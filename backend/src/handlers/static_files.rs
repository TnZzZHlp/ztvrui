use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
};
use include_dir::{include_dir, Dir};

static FRONTEND: Dir = include_dir!("./dist");

pub async fn serve_static_files(path: Uri) -> impl IntoResponse {
    let path = path.to_string();
    let file_path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        &path[1..]
    };

    match FRONTEND.get_file(file_path) {
        Some(file) => {
            let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime_type.as_ref())],
                file.contents(),
            )
                .into_response()
        }
        None => FRONTEND
            .get_file("index.html")
            .map(|file| {
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/html")],
                    file.contents(),
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
    }
}
