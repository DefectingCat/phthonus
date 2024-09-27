use crate::utils::validator::ValidatedJson;

use super::{RouteResponse, RouteResult};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 1, message = "cannot be empyt"))]
    pub username: String,
    pub email: String,
    #[validate(length(min = 1, message = "cannot be empyt"))]
    pub password: String,
}
#[derive(Serialize, Deserialize, Default)]
pub struct RegisterData {
    pub username: String,
}

pub async fn register(
    ValidatedJson(user): ValidatedJson<RegisterPayload>,
) -> RouteResult<RegisterData> {
    let data = RegisterData {
        username: user.username,
    };
    let res = RouteResponse {
        data,
        ..Default::default()
    };
    Ok(Json(res))
}
