use std::{borrow::Cow, convert::Infallible, future::Future, str::FromStr};

use async_graphql::{
    dataloader::DataLoader,
    futures_util::{
        self,
        task::{Context, Poll},
    },
    http::{WebSocketProtocols, WsMessage, ALL_WEBSOCKET_PROTOCOLS},
    Data, Executor, Result,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLWebSocket};
use axum::{
    body::{boxed, BoxBody, HttpBody},
    extract::{
        ws::{CloseFrame, Message},
        FromRequestParts, WebSocketUpgrade,
    },
    http::{self, request::Parts, Request, Response, StatusCode},
    response::IntoResponse,
    Error,
};
use futures_util::{
    future,
    future::{BoxFuture, Ready},
    stream::{SplitSink, SplitStream},
    Sink, SinkExt, Stream, StreamExt,
};
use tower_service::Service;

use crate::AppState;

/// A GraphQL subscription service.
pub struct GraphQLSubscription<E> {
    executor: E,
    app_data: AppState,
}

impl<E> Clone for GraphQLSubscription<E>
where
    E: Executor,
{
    fn clone(&self) -> Self {
        Self {
            executor: self.executor.clone(),
            app_data: self.app_data.clone(),
        }
    }
}

impl<E> GraphQLSubscription<E>
where
    E: Executor,
{
    /// Create a GraphQL subscription service.
    pub fn new(executor: E, app_data: AppState) -> Self {
        Self { executor, app_data }
    }
}

impl<B, E> Service<Request<B>> for GraphQLSubscription<E>
where
    B: HttpBody + Send + 'static,
    E: Executor,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let executor = self.executor.clone();
        let app_data = self.app_data.clone();

        Box::pin(async move {
            let (mut parts, _body) = req.into_parts();

            let protocol = match GraphQLProtocol::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response().map(boxed)),
            };
            let upgrade = match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
                Ok(protocol) => protocol,
                Err(err) => return Ok(err.into_response().map(boxed)),
            };

            let executor = executor.clone();

            let resp = upgrade
                .protocols(ALL_WEBSOCKET_PROTOCOLS)
                .on_upgrade(move |stream| {
                    GraphQLWebSocket::new(stream, executor, protocol)
                        .on_connection_init(|value| {
                            let membership_dataloader =
                                DataLoader::new(app_data.membership_repo, tokio::spawn);
                            let user_dataloader = DataLoader::new(app_data.user_repo, tokio::spawn);
                            let class_dataloader =
                                DataLoader::new(app_data.class_repo, tokio::spawn);
                            let message_dataloader =
                                DataLoader::new(app_data.message_repo, tokio::spawn);
                            let channel_dataloader =
                                DataLoader::new(app_data.channel_repo, tokio::spawn);

                            let mut data = Data::default();
                            data.insert(membership_dataloader);
                            data.insert(user_dataloader);
                            data.insert(class_dataloader);
                            data.insert(message_dataloader);
                            data.insert(channel_dataloader);

                            futures_util::future::ready(Ok(data))
                        })
                        .serve()
                });
            Ok(resp.into_response().map(boxed))
        })
    }
}
