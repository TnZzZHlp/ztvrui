use crate::services::StaticFileService;
use axum::{http::Uri, response::IntoResponse};

pub async fn serve_static_files(path: Uri) -> impl IntoResponse {
    StaticFileService::serve_file(&path.to_string())
}
