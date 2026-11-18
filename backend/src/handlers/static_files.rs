use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};

static FRONTEND: Dir = include_dir!("./dist");

pub async fn serve_static_files(uri: Uri) -> impl IntoResponse {
    let req_path = uri.path().trim_start_matches('/');

    if req_path.split('/').any(|seg| seg == "..") || req_path.contains('\\') {
        return (StatusCode::BAD_REQUEST, "Invalid path").into_response();
    }

    if req_path.is_empty() {
        return serve_index();
    }

    if let Some(file) = FRONTEND.get_file(req_path) {
        let mime_type = mime_guess::from_path(req_path).first_or_octet_stream();
        let cache_control = if req_path.starts_with("assets/") {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };
        return (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, mime_type.as_ref()),
                (header::CACHE_CONTROL, cache_control),
            ],
            file.contents(),
        )
            .into_response();
    }

    let is_missing_asset = req_path.starts_with("assets/")
        || req_path
            .rsplit('/')
            .next()
            .is_some_and(|segment| segment.contains('.'));

    if is_missing_asset {
        return StatusCode::NOT_FOUND.into_response();
    }

    serve_index()
}

fn serve_index() -> Response {
    FRONTEND
        .get_file("index.html")
        .map(|file| {
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                    (header::CACHE_CONTROL, "no-cache"),
                ],
                file.contents(),
            )
                .into_response()
        })
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
