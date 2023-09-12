// https://github.com/meeting-rs/meeting.rs/blob/master/coordinator/src/main.rs
// https://github.com/webrtc-rs/webrtc/tree/master/examples/examples/broadcast

use std::hash::Hash;

use crate::core::{AppError, Claims};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Extension, State, WebSocketUpgrade,
    },
    http::{Request, Response},
    response::IntoResponse,
    Router,
};
use deadpool_redis::{
    redis::{self, aio::PubSub},
    Connection, Pool,
};
use futures_util::SinkExt;
use futures_util::StreamExt;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::sync::mpsc;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{configuration::RTCConfiguration, RTCPeerConnection},
    rtp_transceiver::rtp_codec::RTPCodecType,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum InMessageType {
    Auth {
        token: String,
    },
    Subscribe {
        target_class_id: uuid::Uuid,
    },
    StartMeeting {
        target_class_id: uuid::Uuid,
    },
    JoinMeeting {
        target_class_id: uuid::Uuid,
    },
    SendOffer {
        target_class_id: uuid::Uuid,
        target_user_id: String,
        offer: Map<String, Value>,
    },
    SendAnswer {
        target_class_id: uuid::Uuid,
        target_user_id: String,
        answer: Map<String, Value>,
    },
    SendIceCandidate {
        target_class_id: uuid::Uuid,
        target_user_id: String,
        candidate: Map<String, Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum OutMessageType {
    MeetingStarted,
    UserJoined {
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
    BroadcastUserJoined {
        joined_user_id: String,
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

pub async fn websocket(ws: WebSocketUpgrade, State(redis_pool): State<Pool>) -> impl IntoResponse {
    let conn = redis_pool.get().await.unwrap();
    let conn2 = deadpool_redis::Connection::take(redis_pool.get().await.unwrap());
    let pubsub = conn2.into_pubsub();
    ws.on_upgrade(|socket| handle_socket(socket, conn, pubsub))
}

async fn handle_socket(socket: WebSocket, mut redis_conn: Connection, mut pubsub: PubSub) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let user_id = receiver
        .next()
        .await
        .and_then(|msg| msg.ok())
        .and_then(|msg| match msg {
            Message::Text(msg) => Some(msg),
            _ => None,
        })
        .and_then(|msg| serde_json::from_str::<InMessageType>(&msg).ok())
        .and_then(|msg| match msg {
            InMessageType::Auth { token } => {
                let claims = crate::core::auth::validate_token(&token).ok()?;
                Some(claims.sub)
            }
            _ => None,
        });

    let user_id = match user_id {
        Some(user_id) => user_id,
        None => {
            return;
        }
    };
    let user_id2 = user_id.clone();

    let pubsub_handle = receiver
        .next()
        .await
        .and_then(|msg| msg.ok())
        .and_then(|msg| match msg {
            Message::Text(msg) => Some(msg),
            _ => None,
        })
        .and_then(|msg| serde_json::from_str::<InMessageType>(&msg).ok())
        .and_then(|msg| match msg {
            InMessageType::Subscribe { target_class_id } => {
                let handle = tokio::spawn(async move {
                    pubsub
                        .subscribe(format!("meeting:{}", target_class_id.to_string()))
                        .await
                        .expect("is is possible to subscribe");
                    pubsub
                        .subscribe(format!(
                            "meeting:{}.{}",
                            target_class_id.to_string(),
                            user_id
                        ))
                        .await
                        .expect("is is possible to subscribe");

                    let mut stream = pubsub.on_message();
                    while let Some(msg) = stream.next().await {
                        tracing::debug!("PubSub message: {:?}", msg);
                        let pubsub_msg: SocketHandlerMessage = msg
                            .get_payload()
                            .ok()
                            .and_then(|s: String| serde_json::from_str(s.as_str()).ok())
                            .expect("types are compatible");

                        let pattern: Option<String> =
                            msg.get_pattern().expect("it is possible to get pattern");
                        match pattern {
                            Some(target_user_id) if target_user_id == user_id => {
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
                Some(handle)
            }
            _ => None,
        });

    let mut pubsub_handle = match pubsub_handle {
        Some(pubsub_handle) => pubsub_handle,
        None => {
            return;
        }
    };

    let mut send_task = tokio::spawn(async move {
        let user_id = user_id2;
        tracing::debug!("User {} connected", &user_id);

        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            if let Ok(msg) = serde_json::from_str::<InMessageType>(&msg) {
                tracing::debug!("Message: {:?}", &msg);
                match msg {
                    InMessageType::StartMeeting { target_class_id } => {
                        let meeting_id = uuid::Uuid::new_v4();
                        let message = SocketHandlerMessage::BroadcastMeetingStarted;
                        redis_conn
                            .publish_broadcast(&target_class_id.to_string(), message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("Meeting started: {}", meeting_id);
                    }
                    InMessageType::JoinMeeting { target_class_id } => {
                        let message = SocketHandlerMessage::BroadcastUserJoined {
                            joined_user_id: user_id.clone(),
                        };
                        redis_conn
                            .publish_broadcast(&target_class_id.to_string(), message)
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} joined meeting:{}", user_id, target_class_id);
                    }
                    InMessageType::SendOffer {
                        target_class_id,
                        target_user_id,
                        offer,
                    } => {
                        let message = SocketHandlerMessage::SendOffer {
                            sender_id: user_id.clone(),
                            offer,
                        };
                        redis_conn
                            .publish_directed(
                                &target_class_id.to_string(),
                                &target_user_id,
                                message,
                            )
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} sent offer to {}", user_id, target_user_id);
                    }
                    InMessageType::SendAnswer {
                        target_class_id,
                        target_user_id,
                        answer,
                    } => {
                        let message = SocketHandlerMessage::SendAnswer {
                            sender_id: user_id.clone(),
                            answer,
                        };
                        redis_conn
                            .publish_directed(
                                &target_class_id.to_string(),
                                &target_user_id,
                                message,
                            )
                            .await
                            .expect("it is possible to send");
                        tracing::debug!("User {} sent answer to {}", user_id, target_user_id);
                    }
                    InMessageType::SendIceCandidate {
                        target_class_id,
                        target_user_id,
                        candidate,
                    } => {
                        let message = SocketHandlerMessage::SendIceCandidate {
                            sender_id: user_id.clone(),
                            candidate,
                        };
                        redis_conn
                            .publish_directed(
                                &target_class_id.to_string(),
                                &target_user_id,
                                message,
                            )
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

    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
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
                Ok(SocketHandlerMessage::BroadcastUserJoined { joined_user_id }) => {
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
                Ok(SocketHandlerMessage::SendOffer { sender_id, offer }) => {
                    sender
                        .send(Message::Text(
                            serde_json::to_string(&OutMessageType::Offer { sender_id, offer })
                                .expect("types are compatible"),
                        ))
                        .await
                        .expect("send error");
                }
                Ok(SocketHandlerMessage::SendAnswer { sender_id, answer }) => {
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
                }) => {
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
