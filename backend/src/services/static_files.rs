use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};

static FRONTEND: Dir = include_dir!("./dist");

pub struct StaticFileService;

impl StaticFileService {
    pub fn serve_file(path: &str) -> Response {
        let file_path = if path == "/" || path.is_empty() {
            "index.html"
        } else {
            &path[1..] // Remove leading slash
        };

        match FRONTEND.get_file(file_path) {
            Some(file) => {
                let mime_type = mime_guess::from_path(file_path).first_or_octet_stream();
                let content_type = mime_type.as_ref();

                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, content_type)],
                    file.contents(),
                )
                    .into_response()
            }
            None => {
                // Fallback to index.html for SPA routing
                if let Some(index_file) = FRONTEND.get_file("index.html") {
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "text/html")],
                        index_file.contents(),
                    )
                        .into_response()
                } else {
                    StatusCode::NOT_FOUND.into_response()
                }
            }
        }
    }
}
