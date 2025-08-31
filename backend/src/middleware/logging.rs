use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{error, info, warn};

/// 自定义日志中间件
/// 记录请求的详细信息，包括方法、路径、状态码、耗时等
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let remote_addr = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
        })
        .unwrap_or("unknown")
        .to_string();

    // 记录请求开始
    info!(
        method = %method,
        uri = %uri,
        remote_addr = %remote_addr,
        user_agent = %user_agent,
        "Request started"
    );

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    // 根据状态码选择不同的日志级别
    match status.as_u16() {
        200..=299 => {
            info!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                remote_addr = %remote_addr,
                "Request completed successfully"
            );
        }
        300..=399 => {
            info!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                remote_addr = %remote_addr,
                "Request redirected"
            );
        }
        400..=499 => {
            warn!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                remote_addr = %remote_addr,
                "Client error"
            );
        }
        500..=599 => {
            error!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                remote_addr = %remote_addr,
                "Server error"
            );
        }
        _ => {
            warn!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                remote_addr = %remote_addr,
                "Unknown status code"
            );
        }
    }

    response
}
