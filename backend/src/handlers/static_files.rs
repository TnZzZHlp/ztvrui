use crate::services::StaticFileService;
use axum::{
    extract::Path,
    response::IntoResponse,
};

pub async fn serve_static_files(path: Option<Path<String>>) -> impl IntoResponse {
    let file_path = path.map(|Path(p)| p).unwrap_or_else(|| "/".to_string());
    StaticFileService::serve_file(&file_path)
}
