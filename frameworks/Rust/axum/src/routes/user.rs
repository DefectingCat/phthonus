use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use super::{RouteResponse, RouteResult};

#[derive(Serialize, Deserialize)]
pub struct UserResigtry {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserResigtryRes {
    pub username: String,
    pub email: String,
    pub token: String,
}

pub async fn registry(Json(user_param): Json<UserResigtry>) -> RouteResult<UserResigtryRes> {
    let data = UserResigtryRes {
        username: "xfy".to_string(),
        email: "i@rua.plus".to_string(),
        token: "abc".to_string(),
    };
    let res = RouteResponse {
        data,
        ..Default::default()
    };
    Ok(res)
}

pub fn user_routes() -> Router {
    Router::new().route("/regist", post(registry))
}
