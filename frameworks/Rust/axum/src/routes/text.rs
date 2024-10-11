use axum::response::IntoResponse;

pub async fn text() -> impl IntoResponse {
    "xfy"
}
