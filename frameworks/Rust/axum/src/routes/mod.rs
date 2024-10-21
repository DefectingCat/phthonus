use std::{borrow::Cow, time::Duration};

use axum::{
    http::{StatusCode, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tracing::info;
use user::user_routes;

use crate::{
    error::{AppResult, ErrorCode},
    middlewares::{add_version, logging_route},
};

pub mod json;
pub mod text;
pub mod user;

#[derive(Debug, Serialize)]
pub struct RouteResponse<T>
where
    T: Serialize,
{
    code: ErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
    data: T,
}
impl<T> Default for RouteResponse<T>
where
    T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            code: ErrorCode::Normal,
            message: None,
            data: T::default(),
        }
    }
}
impl<T> IntoResponse for RouteResponse<T>
where
    T: Serialize + Default,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
pub type RouteResult<T> = AppResult<RouteResponse<T>>;

pub fn routes() -> Router {
    let router = Router::new()
        .route("/", get(hello).post(hello))
        .route("/json", get(json::json).post(json::json))
        .route("/text", get(text::text).post(text::text))
        .nest("/user", user_routes())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(add_version))
                .layer(TimeoutLayer::new(Duration::from_secs(15))),
        )
        .fallback(fallback);
    logging_route(router)
}

/// hello world
pub async fn hello() -> String {
    format!("hello {}", env!("CARGO_PKG_NAME"))
}

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    info!("route {} not found", uri);
    (StatusCode::NOT_FOUND, "Not found")
}
