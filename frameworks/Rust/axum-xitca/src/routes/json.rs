use std::borrow::Cow;

use axum::Json;
use serde::{Deserialize, Serialize};

use super::{RouteResponse, RouteResult};

#[derive(Serialize, Deserialize, Default)]
pub struct JsonData {
    pub name: Cow<'static, str>,
}

pub async fn json() -> RouteResult<JsonData> {
    let data = JsonData { name: "xfy".into() };
    let res = RouteResponse {
        data,
        ..Default::default()
    };
    Ok(Json(res))
}
