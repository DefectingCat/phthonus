use std::borrow::Cow;

use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ErrorCode;

use super::{RouteResponse, RouteResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData {
    pub name: Cow<'static, str>,
}

pub async fn json() -> RouteResult<JsonData> {
    let data = JsonData { name: "xfy".into() };
    let res = RouteResponse {
        code: ErrorCode::Normal,
        message: None,
        data,
    };

    Ok(Json(res))
}
