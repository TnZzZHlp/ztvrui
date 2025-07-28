use base64::prelude::*;
use salvo::{http::cookie::Cookie, prelude::*};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::Instant;

use crate::{CONFIG, COOKIE};

/// Verify Cookie legitimacy
#[handler]
pub async fn auth(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    let refuse = |res: &mut Response, ctrl: &mut FlowCtrl| {
        res.status_code(StatusCode::UNAUTHORIZED);
        ctrl.skip_rest();
    };

    let cookie = match req.cookie("Token") {
        Some(cookie) => cookie.value(),
        None => {
            refuse(res, ctrl);
            return;
        }
    };

    let mut cookie_map = COOKIE.write().await;

    if let Some(timestamp) = cookie_map.get_mut(cookie) {
        if timestamp.elapsed() < Duration::from_secs(3600 * 24 * 7) {
            // Cookie is valid
            // Update the timestamp to extend the validity period
            *timestamp = Instant::now();
        } else {
            // Cookie has expired
            cookie_map.remove(cookie);
            refuse(res, ctrl);
            return;
        }
    } else {
        // Cookie not found
        refuse(res, ctrl);
        return;
    }
}

/// Login API
#[handler]
pub async fn login(res: &mut Response, req: &mut Request, ctrl: &mut FlowCtrl) {
    let body = req.parse_json::<serde_json::Value>().await.unwrap();

    let (username, password) = match (body.get("username"), body.get("password")) {
        (Some(username), Some(password)) => {
            (username.as_str().unwrap(), password.as_str().unwrap())
        }
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            ctrl.skip_rest();
            return;
        }
    };

    if CONFIG.read().await.verify(username, password).await {
        let cookie = BASE64_STANDARD.encode(
            format!(
                "{}:{}",
                uuid::Uuid::new_v4(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
            .as_bytes(),
        );

        res.add_header(
            "Set-Cookie",
            Cookie::build(("Token", &cookie))
                .path("/")
                .permanent()
                .build()
                .to_string(),
            true,
        )
        .unwrap();

        res.status_code(StatusCode::NO_CONTENT);

        COOKIE.write().await.insert(cookie, Instant::now());
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
    }
}

/// Logout API
#[handler]
pub async fn logout(_req: &mut Request, res: &mut Response) {
    if let Some(cookie) = _req.cookie("Token") {
        let mut cookie_map = COOKIE.write().await;
        cookie_map.remove(&cookie.to_string());
    }

    res.status_code(StatusCode::NO_CONTENT);
}

/// Modify Username Or Password API
#[handler]
pub async fn modify(res: &mut Response, req: &mut Request) {
    let body = req.parse_json::<serde_json::Value>().await.unwrap();

    let (username, password) = match (body.get("username"), body.get("password")) {
        (Some(username), Some(password)) => {
            (username.as_str().unwrap(), password.as_str().unwrap())
        }
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

    {
        CONFIG
            .write()
            .await
            .update_user_info(username, password)
            .await;
    }

    res.status_code(StatusCode::NO_CONTENT);
}

/// Check Login Status
#[handler]
pub async fn check(res: &mut Response, _req: &mut Request) {
    res.status_code(StatusCode::NO_CONTENT);
}
