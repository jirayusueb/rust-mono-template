use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Global application error — HTTP-status-aligned variants.
/// Per-layer errors convert into this via `From` impls in their own feature modules.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Validation(String), // 400
    #[error("{0}")]
    NotFound(String), // 404
    #[error("{0}")]
    Unauthorized(String), // 401
    #[error("{0}")]
    Conflict(String), // 409
    #[error("{0}")]
    Internal(String), // 500
}

impl AppError {
    /// Stable machine-readable code (per Clean Architecture guide).
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Validation(_) => "ValidationFailed",
            AppError::NotFound(_) => "NotFound",
            AppError::Unauthorized(_) => "Unauthorized",
            AppError::Conflict(_) => "Conflict",
            AppError::Internal(_) => "InternalError",
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        // ponytail: flatten to "field: rule; field: rule" — avoids echoing the
        // submitted values (e.g. password) back in the response, which to_string() does.
        let msg = err
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let rules: Vec<_> = errs.iter().map(|e| e.code.clone()).collect();
                format!("{field}: {}", rules.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        AppError::Validation(msg)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = serde_json::json!({
            "error": {
                "code": self.code(),
                "message": self.to_string(),
            }
        });
        (status, axum::Json(body)).into_response()
    }
}
