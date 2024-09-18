use std::time::Duration;

use axum::{
    body::Bytes,
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span, Span};

use crate::error::AppResult;

/// Middleware for adding version information to each response's headers.
///
/// This middleware takes an incoming `Request` and a `Next` handler, which represents the
/// subsequent middleware or route in the chain. It then asynchronously runs the next handler,
/// obtaining the response. After receiving the response, it appends two headers:
/// - "Server": The name of the server extracted from the Cargo package name.
/// - "S-Version": The version of the server extracted from the Cargo package version.
pub async fn add_version(
    req: Request<axum::body::Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.append("Server", HeaderValue::from_static(env!("CARGO_PKG_NAME")));
    headers.append(
        "Phthonus-Version",
        HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
    );
    Ok(res)
}

/// Middleware for logging each request.
///
/// This middleware will calculate each request latency
/// and add request's information to each info_span.
pub fn logging_route(router: Router) -> Router {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|req: &Request<_>| {
                let unknown = &HeaderValue::from_static("Unknown");
                let empty = &HeaderValue::from_static("");
                let headers = req.headers();
                let ua = headers
                    .get("User-Agent")
                    .unwrap_or(unknown)
                    .to_str()
                    .unwrap_or("Unknown");
                let host = headers.get("Host").unwrap_or(empty).to_str().unwrap_or("");
                info_span!("HTTP", method = ?req.method(), host, uri = ?req.uri(), ua)
            })
            .on_request(|_req: &Request<_>, _span: &Span| {})
            .on_response(|res: &Response, latency: Duration, _span: &Span| {
                info!("{} {}μs", res.status(), latency.as_micros());
            })
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
            .on_eos(|_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {})
            .on_failure(
                |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    error!("{}", error);
                },
            ),
    )
}
