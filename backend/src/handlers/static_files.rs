use std::{collections::HashMap, sync::OnceLock};

use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};

static FRONTEND: Dir<'static> = include_dir!("./dist");
static ETAGS: OnceLock<HashMap<String, HeaderValue>> = OnceLock::new();

fn frontend_etags() -> &'static HashMap<String, HeaderValue> {
    ETAGS.get_or_init(|| build_etag_map(&FRONTEND))
}

fn build_etag_map(dir: &Dir<'_>) -> HashMap<String, HeaderValue> {
    let mut map = HashMap::new();
    collect_etags(dir, &mut map);
    map
}

fn collect_etags(dir: &Dir<'_>, map: &mut HashMap<String, HeaderValue>) {
    for file in dir.files() {
        let path = file.path().to_string_lossy().replace('\\', "/");
        map.insert(path, compute_etag(file.contents()));
    }
    for subdir in dir.dirs() {
        collect_etags(subdir, map);
    }
}

fn compute_etag(bytes: &[u8]) -> HeaderValue {
    const FNV_OFFSET_BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013b;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash ^= u128::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    let tag = format!("\"{:x}-{:032x}\"", bytes.len(), hash);
    HeaderValue::from_str(&tag).expect("hex ETag must be a valid HeaderValue")
}

fn lookup_or_compute_etag(
    etags: &HashMap<String, HeaderValue>,
    path: &str,
    contents: &[u8],
) -> HeaderValue {
    etags
        .get(path)
        .cloned()
        .unwrap_or_else(|| compute_etag(contents))
}

fn strip_weak_prefix(tag: &str) -> &str {
    tag.strip_prefix("W/").unwrap_or(tag)
}

fn matches_if_none_match(headers: &HeaderMap, etag: &str) -> bool {
    let target = strip_weak_prefix(etag.trim());

    for header_value in headers.get_all(header::IF_NONE_MATCH) {
        let Ok(raw) = header_value.to_str() else {
            continue;
        };

        for candidate in raw.split(',').map(str::trim) {
            if candidate == "*" || (!candidate.is_empty() && strip_weak_prefix(candidate) == target)
            {
                return true;
            }
        }
    }

    false
}

pub async fn serve_static_files(method: Method, uri: Uri, headers: HeaderMap) -> impl IntoResponse {
    serve_from_dir(&FRONTEND, frontend_etags(), &method, &uri, &headers)
}

fn serve_from_dir(
    dir: &'static Dir<'static>,
    etags: &HashMap<String, HeaderValue>,
    method: &Method,
    uri: &Uri,
    headers: &HeaderMap,
) -> Response {
    if *method != Method::GET && *method != Method::HEAD {
        return (
            StatusCode::METHOD_NOT_ALLOWED,
            [
                (header::ALLOW, HeaderValue::from_static("GET, HEAD")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
            ],
        )
            .into_response();
    }

    let req_path = uri.path().trim_start_matches('/');

    if req_path.split('/').any(|seg| seg == "..") || req_path.contains('\\') {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CACHE_CONTROL, HeaderValue::from_static("no-store"))],
            "Invalid path",
        )
            .into_response();
    }

    if req_path.is_empty() || req_path == "index.html" {
        return serve_index(dir, etags, method, headers);
    }

    if let Some(file) = dir.get_file(req_path) {
        let mime_type = mime_guess::from_path(req_path).first_or_octet_stream();
        let is_html = mime_type == mime_guess::mime::TEXT_HTML;
        let content_type = if is_html {
            HeaderValue::from_static("text/html; charset=utf-8")
        } else {
            HeaderValue::from_str(mime_type.as_ref())
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
        };
        let cache_control = if req_path.starts_with("assets/") {
            // Vite reserves assets/ for content-hashed outputs; never place unhashed files in frontend/public/assets/.
            HeaderValue::from_static("public, max-age=31536000, immutable")
        } else if is_html {
            HeaderValue::from_static("no-cache")
        } else {
            HeaderValue::from_static("public, max-age=3600")
        };
        let etag = lookup_or_compute_etag(etags, req_path, file.contents());

        return build_file_response(
            method,
            headers,
            file.contents(),
            content_type,
            cache_control,
            etag,
        );
    }

    let is_missing_asset = req_path.starts_with("assets/")
        || req_path
            .rsplit('/')
            .next()
            .is_some_and(|segment| segment.contains('.'));

    if is_missing_asset {
        return not_found_response();
    }

    serve_index(dir, etags, method, headers)
}

fn serve_index(
    dir: &'static Dir<'static>,
    etags: &HashMap<String, HeaderValue>,
    method: &Method,
    headers: &HeaderMap,
) -> Response {
    let Some(file) = dir.get_file("index.html") else {
        return not_found_response();
    };
    let etag = lookup_or_compute_etag(etags, "index.html", file.contents());

    build_file_response(
        method,
        headers,
        file.contents(),
        HeaderValue::from_static("text/html; charset=utf-8"),
        HeaderValue::from_static("no-cache"),
        etag,
    )
}

fn not_found_response() -> Response {
    (
        StatusCode::NOT_FOUND,
        [(header::CACHE_CONTROL, HeaderValue::from_static("no-store"))],
    )
        .into_response()
}

fn build_file_response(
    method: &Method,
    req_headers: &HeaderMap,
    contents: &'static [u8],
    content_type: HeaderValue,
    cache_control: HeaderValue,
    etag: HeaderValue,
) -> Response {
    if etag
        .to_str()
        .is_ok_and(|etag_str| matches_if_none_match(req_headers, etag_str))
    {
        return (
            StatusCode::NOT_MODIFIED,
            [(header::ETAG, etag), (header::CACHE_CONTROL, cache_control)],
        )
            .into_response();
    }

    if *method == Method::HEAD {
        let content_length = HeaderValue::from_str(&contents.len().to_string())
            .expect("content length must be a valid HeaderValue");
        return (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, content_type),
                (header::CACHE_CONTROL, cache_control),
                (header::ETAG, etag),
                (header::CONTENT_LENGTH, content_length),
            ],
            Body::empty(),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, cache_control),
            (header::ETAG, etag),
        ],
        contents,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use include_dir::{DirEntry, File};

    const INDEX_HTML: &[u8] = b"<!doctype html><html><body>ZTVRUI</body></html>";
    const ICON_ICO: &[u8] = b"\x00\x00\x01\x00icon-bytes";
    const APP_JS: &[u8] = b"console.log('ztvrui');";
    const APP_CSS: &[u8] = b"body{margin:0}";

    static TEST_ASSETS_ENTRIES: &[DirEntry<'static>] = &[
        DirEntry::File(File::new("assets/index-DWhZ9gj5.js", APP_JS)),
        DirEntry::File(File::new("assets/index-B-2GPLrC.css", APP_CSS)),
    ];

    static TEST_ROOT_ENTRIES: &[DirEntry<'static>] = &[
        DirEntry::File(File::new("index.html", INDEX_HTML)),
        DirEntry::File(File::new("icon.ico", ICON_ICO)),
        DirEntry::Dir(Dir::new("assets", TEST_ASSETS_ENTRIES)),
    ];

    static TEST_FRONTEND: Dir<'static> = Dir::new("", TEST_ROOT_ENTRIES);

    fn test_etags() -> HashMap<String, HeaderValue> {
        build_etag_map(&TEST_FRONTEND)
    }

    fn send(method: Method, path: &str, headers: HeaderMap) -> Response {
        let etags = test_etags();
        let uri: Uri = path.parse().unwrap();
        serve_from_dir(&TEST_FRONTEND, &etags, &method, &uri, &headers)
    }

    #[tokio::test]
    async fn index_and_spa_routes_share_etag_and_no_cache() {
        let root_resp = send(Method::GET, "/", HeaderMap::new());
        assert_eq!(root_resp.status(), StatusCode::OK);
        assert_eq!(
            root_resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            root_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-cache"
        );
        let root_etag = root_resp.headers().get(header::ETAG).unwrap().clone();
        assert!(root_etag.to_str().unwrap().starts_with('"'));
        assert!(root_etag.to_str().unwrap().ends_with('"'));
        let root_body = to_bytes(root_resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(root_body.as_ref(), INDEX_HTML);

        for route in [
            "/index.html",
            "/networks",
            "/network/0123456789abcdef/members",
        ] {
            let resp = send(Method::GET, route, HeaderMap::new());
            assert_eq!(resp.status(), StatusCode::OK, "route {route}");
            assert_eq!(
                resp.headers().get(header::CONTENT_TYPE).unwrap(),
                "text/html; charset=utf-8"
            );
            assert_eq!(
                resp.headers().get(header::CACHE_CONTROL).unwrap(),
                "no-cache"
            );
            assert_eq!(resp.headers().get(header::ETAG).unwrap(), &root_etag);
            let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
            assert_eq!(body.as_ref(), INDEX_HTML);
        }
    }

    #[tokio::test]
    async fn if_none_match_variants_return_304_or_200() {
        let initial = send(Method::GET, "/index.html", HeaderMap::new());
        let etag = initial
            .headers()
            .get(header::ETAG)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        for candidate in [
            etag.clone(),
            format!("W/{etag}"),
            format!("\"other\", W/{etag}"),
            "*".to_string(),
        ] {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::IF_NONE_MATCH,
                HeaderValue::from_str(&candidate).unwrap(),
            );
            let resp = send(Method::GET, "/networks", headers);
            assert_eq!(
                resp.status(),
                StatusCode::NOT_MODIFIED,
                "candidate {candidate}"
            );
            assert_eq!(resp.headers().get(header::ETAG).unwrap(), &etag);
            assert_eq!(
                resp.headers().get(header::CACHE_CONTROL).unwrap(),
                "no-cache"
            );
            let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
            assert!(body.is_empty());
        }

        let mut mismatch_headers = HeaderMap::new();
        mismatch_headers.insert(header::IF_NONE_MATCH, HeaderValue::from_static("\"stale\""));
        let mismatch_resp = send(Method::GET, "/index.html", mismatch_headers);
        assert_eq!(mismatch_resp.status(), StatusCode::OK);
        let body = to_bytes(mismatch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body.as_ref(), INDEX_HTML);
    }

    #[tokio::test]
    async fn hashed_assets_and_public_files_have_appropriate_cache_policies() {
        let js_resp = send(Method::GET, "/assets/index-DWhZ9gj5.js", HeaderMap::new());
        assert_eq!(js_resp.status(), StatusCode::OK);
        assert_eq!(
            js_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=31536000, immutable"
        );
        let js_etag = js_resp.headers().get(header::ETAG).unwrap().clone();
        let js_body = to_bytes(js_resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(js_body.as_ref(), APP_JS);

        let mut js_cond_headers = HeaderMap::new();
        js_cond_headers.insert(header::IF_NONE_MATCH, js_etag);
        let js_304 = send(Method::GET, "/assets/index-DWhZ9gj5.js", js_cond_headers);
        assert_eq!(js_304.status(), StatusCode::NOT_MODIFIED);

        let css_resp = send(Method::GET, "/assets/index-B-2GPLrC.css", HeaderMap::new());
        assert_eq!(css_resp.status(), StatusCode::OK);
        assert_eq!(
            css_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=31536000, immutable"
        );
        assert!(css_resp.headers().get(header::ETAG).is_some());

        let ico_resp = send(Method::GET, "/icon.ico", HeaderMap::new());
        assert_eq!(ico_resp.status(), StatusCode::OK);
        assert_eq!(
            ico_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=3600"
        );
        let ico_etag = ico_resp.headers().get(header::ETAG).unwrap().clone();
        let ico_body = to_bytes(ico_resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(ico_body.as_ref(), ICON_ICO);

        let mut ico_cond_headers = HeaderMap::new();
        ico_cond_headers.insert(header::IF_NONE_MATCH, ico_etag);
        let ico_304 = send(Method::GET, "/icon.ico", ico_cond_headers);
        assert_eq!(ico_304.status(), StatusCode::NOT_MODIFIED);
    }

    #[tokio::test]
    async fn head_requests_return_headers_without_body_and_support_304() {
        let head_index = send(Method::HEAD, "/index.html", HeaderMap::new());
        assert_eq!(head_index.status(), StatusCode::OK);
        assert_eq!(
            head_index.headers().get(header::CONTENT_LENGTH).unwrap(),
            &INDEX_HTML.len().to_string()
        );
        assert_eq!(
            head_index.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-cache"
        );
        let etag = head_index.headers().get(header::ETAG).unwrap().clone();
        let body = to_bytes(head_index.into_body(), usize::MAX).await.unwrap();
        assert!(body.is_empty());

        let mut cond_headers = HeaderMap::new();
        cond_headers.insert(header::IF_NONE_MATCH, etag);
        let head_304 = send(Method::HEAD, "/index.html", cond_headers);
        assert_eq!(head_304.status(), StatusCode::NOT_MODIFIED);
        let body_304 = to_bytes(head_304.into_body(), usize::MAX).await.unwrap();
        assert!(body_304.is_empty());

        let head_ico = send(Method::HEAD, "/icon.ico", HeaderMap::new());
        assert_eq!(head_ico.status(), StatusCode::OK);
        assert_eq!(
            head_ico.headers().get(header::CONTENT_LENGTH).unwrap(),
            &ICON_ICO.len().to_string()
        );
        assert_eq!(
            head_ico.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=3600"
        );
        let ico_body = to_bytes(head_ico.into_body(), usize::MAX).await.unwrap();
        assert!(ico_body.is_empty());
    }

    #[tokio::test]
    async fn error_and_disallowed_method_responses_use_no_store() {
        for missing in ["/assets/missing.js", "/missing.png"] {
            let resp = send(Method::GET, missing, HeaderMap::new());
            assert_eq!(resp.status(), StatusCode::NOT_FOUND, "{missing}");
            assert_eq!(
                resp.headers().get(header::CACHE_CONTROL).unwrap(),
                "no-store"
            );
        }

        let bad_resp = send(Method::GET, "/../secret", HeaderMap::new());
        assert_eq!(bad_resp.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            bad_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-store"
        );

        let method_resp = send(Method::POST, "/networks", HeaderMap::new());
        assert_eq!(method_resp.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            method_resp.headers().get(header::ALLOW).unwrap(),
            "GET, HEAD"
        );
        assert_eq!(
            method_resp.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-store"
        );
    }
}
