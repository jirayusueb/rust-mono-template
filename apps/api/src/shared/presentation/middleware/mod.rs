use axum::http::{Request, Response};
use axum::middleware::Next;
use uuid::Uuid;

/// Generates a request ID, stamps the response header, and records a tracing span
/// so all logs within this request carry the correlation ID.
pub async fn request_id(req: Request<axum::body::Body>, next: Next) -> Response<axum::body::Body> {
    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let span = tracing::info_span!("request", request_id = %request_id);
    let mut resp = span.in_scope(|| async move { next.run(req).await }).await;

    resp.headers_mut()
        .insert("x-request-id", request_id.parse().unwrap());
    resp
}
