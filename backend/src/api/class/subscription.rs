use crate::api::assignment::AssignmentObject;
use crate::api::channel::ChannelObject;
use crate::api::class::ClassObject;
use crate::api::file::FileObject;

use crate::api::user::UserObject;
use crate::core::AppError;
use async_graphql::futures_util::StreamExt;
use async_graphql::{futures_util::Stream, Context, Subscription};
use async_graphql::{SimpleObject, Union, ID};
use deadpool_redis::Pool;
use paste::paste;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub const CLASS_RESOURCE_CREATED: &str = "class_resource_created";
pub const CLASS_RESOURCE_UPDATED: &str = "class_resource_updated";
pub const CLASS_RESOURCE_DELETED: &str = "class_resource_deleted";

#[derive(Default)]
pub struct ClassSubscription;

#[Subscription]
impl ClassSubscription {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn class_resource_created(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<impl Stream<Item = ClassResourceCreate>, AppError> {
        make_subscription(
            ctx,
            format!("{}:{}", CLASS_RESOURCE_CREATED, class_id.as_str()),
        )
        .await
    }

    async fn class_resource_updated(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<impl Stream<Item = ClassResourceUpdate>, AppError> {
        make_subscription(
            ctx,
            format!("{}:{}", CLASS_RESOURCE_UPDATED, class_id.as_str()),
        )
        .await
    }

    async fn class_resource_deleted(
        &self,
        ctx: &Context<'_>,
        class_id: ID,
    ) -> Result<impl Stream<Item = ClassResourceDelete>, AppError> {
        make_subscription(
            ctx,
            format!("{}:{}", CLASS_RESOURCE_DELETED, class_id.as_str()),
        )
        .await
    }
}

async fn make_subscription<T: DeserializeOwned>(
    ctx: &Context<'_>,
    channel: String,
) -> Result<impl Stream<Item = T>, AppError> {
    let redis_pool = ctx.data_unchecked::<Pool>();
    let conn = deadpool_redis::Connection::take(redis_pool.get().await?);
    let mut conn = conn.into_pubsub();

    conn.subscribe(channel).await?;
    Ok(conn.into_on_message().filter_map(|msg| async move {
        msg.get_payload()
            .ok()
            .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
    }))
}

#[derive(Debug, Serialize, Deserialize, Union)]
pub enum ClassResourceCreate {
    Channel(ChannelObject),
    Member(UserObject),
    File(FileObject),
    Assignment(AssignmentObject),
}

#[derive(Debug, Serialize, Deserialize, Union)]
pub enum ClassResourceUpdate {
    Class(ClassObject),
    Channel(ChannelObject),
    Assignment(AssignmentObject),
}

#[derive(Debug, Serialize, Deserialize, Union)]
pub enum ClassResourceDelete {
    Channel(ChannelDeleteInfo),
    Member(MemberDeleteInfo),
    File(FileDeleteInfo),
    Assignment(AssignmentDeleteInfo),
}

macro_rules! make_a_struct {
    ($name:ident, $from:ty) => {
        paste! {
            #[derive(Debug, Serialize, Deserialize, SimpleObject)]
            pub struct [<$name DeleteInfo>] {
                pub id: ID,
            }

            impl From<$from> for [<$name DeleteInfo>] {
                fn from(model: $from) -> Self {
                    Self { id: model.id.into() }
                }
            }
        }
    };
}

make_a_struct!(Channel, entity::channel::Model);
make_a_struct!(File, entity::file::Model);
make_a_struct!(Assignment, entity::assignment::Model);
make_a_struct!(Member, entity::user::Model);
