use std::collections::HashMap;

use crate::{
    api::ClassRepo,
    core::{AppError, Claims},
};
use async_graphql::dataloader::DataLoader;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    Json,
};
use deadpool_redis::{
    redis::{self, aio::PubSub},
    Connection, Pool,
};
use futures_util::SinkExt;
use futures_util::StreamExt;
use redis::AsyncCommands;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing::instrument;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum InMessageType {
    Auth {
        token: String,
        class_id: String,
    },
    StartMeeting,
    StopMeeting,
    JoinMeeting,
    LeaveMeeting,
    SendOffer {
        target_user_id: String,
        offer: Map<String, Value>,
    },
    SendAnswer {
        target_user_id: String,
        answer: Map<String, Value>,
    },
    SendIceCandidate {
        target_user_id: String,
        candidate: Map<String, Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum OutMessageType {
    MeetingStarted,
    MeetingStopped,
    UserJoined {
        user_id: String,
    },
    UserLeft {
        user_id: String,
    },
    Offer {
        sender_id: String,
        offer: Map<String, Value>,
    },
    Answer {
        sender_id: String,
        answer: Map<String, Value>,
    },
    IceCandidate {
        sender_id: String,
        candidate: Map<String, Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum SocketHandlerMessage {
    BroadcastMeetingStarted,
    BroadcastMeetingStopped,
    BroadcastUserJoined {
        joined_user_id: String,
    },
    BroadcastUserLeft {
        left_user_id: String,
    },
    SendOffer {
        sender_id: String,
        offer: Map<String, Value>,
    },
    SendAnswer {
        sender_id: String,
        answer: Map<String, Value>,
    },
    SendIceCandidate {
        sender_id: String,
        candidate: Map<String, Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingData {
    peer_ids: Vec<String>,
}

pub async fn current_meeting(
    State(redis_pool): State<Pool>,
    Path(class_id): Path<uuid::Uuid>,
    claims: Claims,
) -> Result<Json<MeetingData>, AppError> {
    let mut conn = redis_pool.get().await.unwrap();
    let user_id = claims.sub;
    let peer_ids: HashMap<String, u32> = conn.hgetall(format!("meeting:{}", class_id)).await?;
    let filtered = peer_ids
        .into_iter()
        .filter(|(_, v)| *v == 1)
        .filter(|(k, _)| k != &user_id)
        .map(|(k, _)| k)
        .collect();

    Ok(MeetingData { peer_ids: filtered }.into())
}

pub async fn websocket(
    ws: WebSocketUpgrade,
    State(redis_pool): State<Pool>,
    State(conn): State<DatabaseConnection>,
) -> impl IntoResponse {
    let redis_conn = deadpool_redis::Connection::take(redis_pool.get().await.unwrap());
    let pubsub = redis_conn.into_pubsub();
    let data_loader = DataLoader::new(conn, tokio::spawn);
    ws.on_upgrade(|socket| handle_socket_wrapper(socket, redis_pool, pubsub, data_loader))
}

async fn handle_socket_wrapper(
    socket: WebSocket,
    redis_pool: Pool,
    pubsub: PubSub,
    data_loader: DataLoader<DatabaseConnection>,
) {
    let _ = handle_socket(socket, redis_pool, pubsub, data_loader).await;
}

#[instrument(skip(socket, redis_pool, pubsub, data_loader), err)]
async fn handle_socket(
    socket: WebSocket,
    redis_pool: Pool,
    mut pubsub: PubSub,
    data_loader: DataLoader<DatabaseConnection>,
) -> Result<(), AppError> {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let mut redis_conn = redis_pool.get().await?;

    let msg = receiver
        .next()
        .await
        .and_then(|msg| msg.ok())
        .and_then(|msg| match msg {
            Message::Text(msg) => Some(msg),
            _ => None,
        })
        .and_then(|msg| serde_json::from_str::<InMessageType>(&msg).ok())
        .ok_or(AppError::auth("invalid first message"))?;

    let connection_data = match msg {
        InMessageType::Auth { token, class_id } => {
            let claims = crate::core::auth::validate_token(&token)?;
            let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).expect("id is valid uuid");
            let class_id = uuid::Uuid::parse_str(class_id.as_str())?;

            let cloned_user_id = claims.sub.clone();
            let cloned_class_id = class_id.to_string();

            let class = ClassRepo::find_by_id(&data_loader, class_id)
                .await?
                .ok_or_else(|| AppError::auth("invalid first message"))?;

            let is_owner = class.owner_id == user_id;
            let members = ClassRepo::get_members(&data_loader, class_id).await?;
            let is_member = members.iter().any(|member| member.id == user_id);

            if !is_member {
                return Err(AppError::auth("invalid first message"));
            }

            let handle = tokio::spawn(async move {
                pubsub
                    .subscribe(format!("meeting:{}", class_id))
                    .await
                    .expect("is is possible to subscribe");
                pubsub
                    .subscribe(format!("meeting:{}.{}", class_id, user_id))
                    .await
                    .expect("is is possible to subscribe");

                let _: u32 = redis_conn
                    .hset(format!("meeting:{}", class_id), user_id.to_string(), 0)
                    .await
                    .expect("it is possible to set");

                let mut stream = pubsub.on_message();
                while let Some(msg) = stream.next().await {
                    let pubsub_msg: SocketHandlerMessage = msg
                        .get_payload()
                        .ok()
                        .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
                        .expect("types are compatible");

                    let pattern: Option<String> =
                        msg.get_pattern().expect("it is possible to get pattern");
                    match pattern {
                        Some(target_user_id) if target_user_id == user_id.to_string() => {
                            // directed message
                            tx.send(Ok::<_, AppError>(pubsub_msg))
                                .await
                                .expect("receiver is not dropped");
                        }
                        Some(_) => {
                            // ignore
                        }
                        None => {
                            // broadcast message
                            tx.send(Ok::<_, AppError>(pubsub_msg))
                                .await
                                .expect("receiver is not dropped");
                        }
                    }
                }
            });

            Some((cloned_user_id, cloned_class_id, is_owner, handle))
        }
        _ => None,
    };

    let (user_id, class_id, is_owner, mut pubsub_handle) =
        connection_data.ok_or(AppError::auth("invalid first message"))?;
    let cloned_user_id = user_id.clone();
    let cloned_class_id = class_id.clone();
    let mut redis_conn = redis_pool.get().await?;

    let mut send_task = tokio::spawn(async move {
        let user_id = cloned_user_id;
        let class_id = cloned_class_id;
        tracing::debug!("User {} connected", &user_id);

        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            if let Ok(msg) = serde_json::from_str::<InMessageType>(&msg) {
                match msg {
                    InMessageType::StartMeeting if is_owner => {
                        let _: u32 = redis_conn
                            .hset(format!("meeting:{}", class_id), user_id.clone(), 1)
                            .await
                            .expect("it is possible to set");
                        let message = SocketHandlerMessage::BroadcastMeetingStarted;
                        redis_conn
                            .publish_broadcast(&class_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("Meeting started: {}", &class_id);
                    }
                    InMessageType::StopMeeting => {
                        let _: u32 = redis_conn
                            .hdel("meeting", &class_id)
                            .await
                            .expect("it is possible to remove");
                        let message = SocketHandlerMessage::BroadcastMeetingStopped;
                        redis_conn
                            .publish_broadcast(&class_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("Meeting stopped: {}", &class_id);
                    }
                    InMessageType::JoinMeeting => {
                        let _: u32 = redis_conn
                            .hset(format!("meeting:{}", class_id), user_id.clone(), 1)
                            .await
                            .expect("it is possible to set");
                        let message = SocketHandlerMessage::BroadcastUserJoined {
                            joined_user_id: user_id.clone(),
                        };
                        redis_conn
                            .publish_broadcast(&class_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} joined meeting:{}", user_id, class_id);
                    }
                    InMessageType::LeaveMeeting => {
                        let _: u32 = redis_conn
                            .hset(format!("meeting:{}", class_id), &user_id, 0)
                            .await
                            .expect("it is possible to remove");
                        let message = SocketHandlerMessage::BroadcastUserLeft {
                            left_user_id: user_id.clone(),
                        };
                        redis_conn
                            .publish_broadcast(&class_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} left meeting:{}", user_id, class_id);
                    }
                    InMessageType::SendOffer {
                        target_user_id,
                        offer,
                    } => {
                        let message = SocketHandlerMessage::SendOffer {
                            sender_id: user_id.clone(),
                            offer,
                        };
                        redis_conn
                            .publish_directed(&class_id, &target_user_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} sent offer to {}", user_id, target_user_id);
                    }
                    InMessageType::SendAnswer {
                        target_user_id,
                        answer,
                    } => {
                        let message = SocketHandlerMessage::SendAnswer {
                            sender_id: user_id.clone(),
                            answer,
                        };
                        redis_conn
                            .publish_directed(&class_id, &target_user_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} sent answer to {}", user_id, target_user_id);
                    }
                    InMessageType::SendIceCandidate {
                        target_user_id,
                        candidate,
                    } => {
                        let message = SocketHandlerMessage::SendIceCandidate {
                            sender_id: user_id.clone(),
                            candidate,
                        };
                        redis_conn
                            .publish_directed(&class_id, &target_user_id, message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!(
                            "User {} sent ice candidate to {}",
                            user_id,
                            target_user_id
                        );
                    }
                    _ => {
                        tracing::warn!("StartMeeting and Subscribe sent more than once")
                    }
                }
            } else {
                tracing::warn!("Invalid message: {}", &msg);
            }
        }
    });

    let cloned_user_id = user_id.clone();
    let cloned_class_id = class_id.clone();
    let mut redis_conn = redis_pool.get().await?;

    let mut recv_task = tokio::spawn(async move {
        let user_id = cloned_user_id;
        let class_id = cloned_class_id;
        while let Some(msg) = rx.recv().await {
            let is_user_joined = redis_conn
                .hget::<_, _, u32>(format!("meeting:{}", class_id), &user_id)
                .await
                .expect("it is possible to get")
                == 1;
            match msg {
                Ok(SocketHandlerMessage::BroadcastMeetingStarted) => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::MeetingStarted)
                                .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::BroadcastMeetingStopped) => {
                    let _: u32 = redis_conn
                        .hset(format!("meeting:{}", class_id), &user_id, 0)
                        .await
                        .expect("it is possible to send");

                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::MeetingStopped)
                                .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::BroadcastUserJoined { joined_user_id })
                    if is_user_joined =>
                {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::UserJoined {
                                user_id: joined_user_id,
                            })
                            .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::BroadcastUserLeft { left_user_id }) if is_user_joined => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::UserLeft {
                                user_id: left_user_id,
                            })
                            .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::SendOffer { sender_id, offer }) if is_user_joined => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::Offer { sender_id, offer })
                                .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::SendAnswer { sender_id, answer }) if is_user_joined => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::Answer { sender_id, answer })
                                .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::SendIceCandidate {
                    sender_id,
                    candidate,
                }) if is_user_joined => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::IceCandidate {
                                sender_id,
                                candidate,
                            })
                            .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(_) => {}
                Err(_) => {
                    sender
                        .send(Message::Text("Error".to_string()))
                        .await
                        .expect("send error");
                }
            }
        }
    });

    tokio::select! {
        _ =(&mut send_task) => {
            recv_task.abort();
            pubsub_handle.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
            pubsub_handle.abort();
        },
        _ = (&mut pubsub_handle) => {
            send_task.abort();
            recv_task.abort();
        }
    }
    let mut redis_conn = redis_pool.get().await?;

    if is_owner {
        let message = SocketHandlerMessage::BroadcastMeetingStopped;
        redis_conn
            .publish_broadcast(&class_id, message)
            .await
            .expect("it is possible to send");
    } else {
        let _: u32 = redis_conn
            .hdel(format!("meeting:{}", class_id), &user_id)
            .await
            .expect("it is possible to remove");
    }
    Ok(())
}

#[async_trait::async_trait]
trait PubSubExt {
    async fn publish_directed(
        &mut self,
        target_class_id: &String,
        target_user_id: &String,
        message: SocketHandlerMessage,
    ) -> Result<(), AppError>;

    async fn publish_broadcast(
        &mut self,
        target_class_id: &String,
        message: SocketHandlerMessage,
    ) -> Result<(), AppError>;
}

#[async_trait::async_trait]
impl PubSubExt for Connection {
    async fn publish_directed(
        &mut self,
        target_class_id: &String,
        target_user_id: &String,
        message: SocketHandlerMessage,
    ) -> Result<(), AppError> {
        let _: u32 = self
            .publish(
                format!("meeting:{}.{}", target_class_id, target_user_id),
                serde_json::to_string(&message).expect("this is always valid"),
            )
            .await?;
        Ok(())
    }

    async fn publish_broadcast(
        &mut self,
        target_class_id: &String,
        message: SocketHandlerMessage,
    ) -> Result<(), AppError> {
        let _: u32 = self
            .publish(
                format!("meeting:{}", target_class_id),
                serde_json::to_string(&message).expect("this is always valid"),
            )
            .await?;
        Ok(())
    }
}
