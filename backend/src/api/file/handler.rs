use async_graphql::dataloader::DataLoader;
use axum::{
    body::Body,
    extract::{Path, State},
    http::Response,
    response::IntoResponse,
    Json,
};

use entity::{file, sea_orm_active_enums::FileType};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use uuid::Uuid;
use zip::write::FileOptions;
use zip::CompressionMethod;

use crate::{api::file::FileRepo, core::AppError};

pub struct FileHandler;

#[derive(serde::Deserialize, Debug)]
pub struct GetClassFilesPayload {
    file_ids: Vec<Uuid>,
}

impl FileHandler {
    #[instrument(skip(s3_bucket), err(Debug))]
    pub async fn get_user_avatar(
        Path(user_id): Path<Uuid>,
        State(s3_bucket): State<s3::Bucket>,
    ) -> Result<impl IntoResponse, AppError> {
        let s3_path = format!("user-avatars/{user_id}");
        let object = s3_bucket.get_object(s3_path).await;
        match object {
            Ok(object) => {
                let response = Response::builder()
                    .header("Content-Type", "image/jpeg")
                    .body(Body::from(object.to_vec()))
                    .unwrap();
                Ok(response)
            }
            Err(s3::error::S3Error::HttpFailWithBody(404, _)) => {
                let s3_path = "user-avatars/user.png".to_string();
                let object = s3_bucket.get_object(s3_path).await;
                match object {
                    Ok(object) => {
                        let response = Response::builder()
                            .header("Content-Type", "image/jpeg")
                            .body(Body::from(object.to_vec()))
                            .unwrap();
                        Ok(response)
                    }
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(s3_bucket), err(Debug))]
    pub async fn get_class_image(
        Path(class_id): Path<Uuid>,
        State(s3_bucket): State<s3::Bucket>,
    ) -> Result<impl IntoResponse, AppError> {
        let s3_path = format!("class-images/{class_id}");
        let object = s3_bucket.get_object(s3_path).await;
        match object {
            Ok(object) => {
                let response = Response::builder()
                    .header("Content-Type", "image/jpeg")
                    .body(Body::from(object.to_vec()))
                    .unwrap();

                Ok(response)
            }
            Err(s3::error::S3Error::HttpFailWithBody(404, _)) => Err(AppError::not_found(
                "Class image not found".into(),
                "class image",
                "id",
                class_id.to_string(),
            )),
            Err(e) => Err(e.into()),
        }
    }

    // TODO: Add authorization
    #[instrument(skip(s3_bucket), err(Debug))]
    pub async fn get_class_file(
        Path((class_id, file_id)): Path<(Uuid, Uuid)>,
        State(s3_bucket): State<s3::Bucket>,
    ) -> Result<impl IntoResponse, AppError> {
        let s3_path = format!("class-files/{class_id}/{file_id}");
        let object = s3_bucket.get_object(s3_path).await;
        match object {
            Ok(object) => {
                let def = "application/octet-stream".to_owned();
                let headers = object.headers();
                let ct = headers.get("content-type").unwrap_or(&def);

                let response = Response::builder()
                    .header("Content-Type", ct)
                    .body(Body::from(object.to_vec()))
                    .unwrap();

                Ok(response)
            }
            Err(s3::error::S3Error::HttpFailWithBody(404, _)) => Err(AppError::not_found(
                "Class file not found".into(),
                "class file",
                "id",
                class_id.to_string(),
            )),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(s3_bucket, conn), err(Debug))]
    pub async fn get_class_files(
        Path(class_id): Path<Uuid>,
        State(s3_bucket): State<s3::Bucket>,
        State(conn): State<DatabaseConnection>,
        Json(payload): Json<GetClassFilesPayload>,
    ) -> Result<impl IntoResponse, AppError> {
        let data_loader = DataLoader::new(conn, tokio::spawn);

        let files = FileRepo::find_many_with_nested(&data_loader, payload.file_ids).await?;
        let zip_data = create_zip_archive(files, &s3_bucket, &class_id.to_string())
            .await
            .unwrap();
        Ok(axum::body::Bytes::from(zip_data).into_response())
    }
}

async fn create_zip_archive(
    files: Vec<file::Model>,
    s3_bucket: &s3::Bucket,
    class_id: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::io::Write;
    let _options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut zip_buffer = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        for file in &files {
            let file_path = get_file_path(&files, file);

            if file.file_type == FileType::Directory {
                let options = FileOptions::default()
                    .compression_method(CompressionMethod::Stored)
                    .unix_permissions(0o755);

                zip.add_directory(file_path, options)
                    .expect("Failed to add directory to zip archive");
            } else {
                let options = FileOptions::default()
                    .compression_method(CompressionMethod::Deflated)
                    .unix_permissions(0o644);

                zip.start_file(&file_path, options)
                    .expect("Failed to create file in zip archive");

                let s3_path = format!("class-files/{class_id}/{file_path}");
                let object = s3_bucket.get_object(s3_path).await;
                if let Ok(object) = object {
                    zip.write_all(&object.to_vec())
                        .expect("Failed to write file to zip archive");
                }
            }
        }

        zip.finish().expect("Failed to finish writing zip archive");
    }

    Ok(zip_buffer)
}

fn create_zip_directory_structure(
    _files: Vec<file::Model>,
    _zip: &mut zip::ZipWriter<std::io::Cursor<&mut [u8]>>,
) {
}

fn get_file_path(files: &[file::Model], file: &file::Model) -> String {
    let mut path = file.name.clone();
    let mut parent_id = file.parent_id;

    while let Some(id) = parent_id {
        if let Some(parent) = files.iter().find(|f| f.id == id) {
            path = format!("{}/{}", parent.name, path);
            parent_id = parent.parent_id;
        } else {
            break;
        }
    }

    path
}
