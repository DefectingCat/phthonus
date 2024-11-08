use crate::utils::password::hash;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validator::EMAIL_REGEX;

use super::{RouteResponse, RouteResult};

#[derive(Serialize, Deserialize, Validate)]
pub struct UserResigtry {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(regex(
        path = *EMAIL_REGEX,
        message = "邮箱格式不正确"
    ))]
    pub email: String,
    #[validate(length(min = 6, max = 100, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserResigtryRes {
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
}

pub async fn registry(Json(user_param): Json<UserResigtry>) -> RouteResult<UserResigtryRes> {
    let UserResigtry {
        email,
        password,
        username,
    } = user_param;

    let hashed = hash(password).await?;

    let data = UserResigtryRes {
        username,
        email,
        password: hashed,
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