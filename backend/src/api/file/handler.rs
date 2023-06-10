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
use zip::ZipWriter;

use crate::{api::file::FileRepo, core::AppError};

pub struct FileHandler;

#[derive(serde::Deserialize, Debug)]
pub struct GetClassFilesPayload {
    file_ids: Vec<Uuid>,
}

impl FileHandler {
    #[instrument(err, skip(s3_bucket))]
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
            Err(s3::error::S3Error::Http(404, _)) => Err(AppError::NotFound {
                what: "user avatar",
                with: "user id",
                why: user_id.to_string(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(err, skip(s3_bucket))]
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
            Err(s3::error::S3Error::Http(404, _)) => Err(AppError::NotFound {
                what: "class image",
                with: "class id",
                why: class_id.to_string(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(err, skip(s3_bucket))]
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
            Err(s3::error::S3Error::Http(404, _)) => Err(AppError::NotFound {
                what: "class file",
                with: "class_id",
                why: class_id.to_string(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(err, skip(s3_bucket, conn))]
    pub async fn get_class_files(
        Path(class_id): Path<Uuid>,
        State(s3_bucket): State<s3::Bucket>,
        State(conn): State<DatabaseConnection>,
        Json(payload): Json<GetClassFilesPayload>,
    ) -> Result<impl IntoResponse, AppError> {
        // let mut buf = Vec::new();
        // let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf[..]));
        // let options =
        //     zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let data_loader = DataLoader::new(conn, tokio::spawn);
        let files_data = FileRepo::find_many_with_nested(&data_loader, payload.file_ids).await?;
        tracing::debug!("files_data: {:?}", files_data);

        // let zip_path = "directory_structure.zip";
        // create_zip_directory_structure(files_data, zip_path);

        Ok("ok")
    }
}

fn create_zip_directory_structure(files: Vec<file::Model>, zip_path: &str) {
    let mut zip = ZipWriter::new(std::fs::File::create(zip_path).unwrap());

    let c = files.clone();
    for file in files {
        let file_path = get_file_path(&c, &file);

        if file.file_type == FileType::Directory {
            // Create directory entry in the zip archive
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755); // Set appropriate permissions for directories

            zip.add_directory(file_path, options)
                .expect("Failed to add directory to zip archive");
        } else {
            // Create file entry in the zip archive
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Deflated) // You can choose the desired compression method
                .unix_permissions(0o644); // Set appropriate permissions for files

            zip.start_file(file_path, options)
                .expect("Failed to create file in zip archive");
        }
    }

    zip.finish().expect("Failed to finish writing zip archive");
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
