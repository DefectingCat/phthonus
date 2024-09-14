use std::{borrow::Cow, fmt::Display};

use axum::{
    extract::rejection::{FormRejection, JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use serde_repr::*;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Any(#[from] anyhow::Error),

    // axum
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    // route
    // 路由通常错误 错误信息直接返回用户
    #[error("{0}")]
    AuthorizeFailed(Cow<'static, str>),
    #[error("{0}")]
    UserConflict(Cow<'static, str>),
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ErrorCode {
    Normal = 200,
    InternalError = 1000,
    //NotAuthorized = 1001,
    AuthorizeFailed = 1002,
    UserConflict = 1003,
    ParameterIncorrect = 1004,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorCode::*;

        let res = match self {
            Normal => "",
            InternalError => "服务器内部错误",
            //NotAuthorized => "未登录",
            AuthorizeFailed => "用户名或密码错误",
            UserConflict => "该用户已经存在",
            ParameterIncorrect => "请求参数错误",
        };
        f.write_str(res)?;
        Ok(())
    }
}

/// Log and return INTERNAL_SERVER_ERROR
fn log_internal_error<T: Display>(err: T) -> (StatusCode, ErrorCode, String) {
    use ErrorCode::*;

    error!("{err}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        InternalError,
        "internal server error".to_string(),
    )
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use ErrorCode::*;

        let (status_code, code, err_message) = match self {
            AppError::Any(err) => log_internal_error(err),
            AppError::AxumFormRejection(_) | AppError::AxumJsonRejection(_) => (
                StatusCode::BAD_REQUEST,
                ParameterIncorrect,
                self.to_string(),
            ),

            // route
            AppError::AuthorizeFailed(err) => {
                (StatusCode::UNAUTHORIZED, AuthorizeFailed, err.to_string())
            }
            AppError::UserConflict(err) => (StatusCode::CONFLICT, UserConflict, err.to_string()),
        };
        let body = Json(json!({
            "code": code,
            "message": code.to_string(),
            "error": err_message
        }));
        (status_code, body).into_response()
    }
}

pub type AppResult<T, E = AppError> = Result<T, E>;
