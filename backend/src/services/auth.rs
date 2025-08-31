use base64::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    sessions: Arc<RwLock<HashMap<String, Instant>>>,
    session_duration: Duration,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_duration: Duration::from_secs(3600 * 24 * 7), // 7 days
        }
    }

    pub async fn create_session(&self) -> String {
        let session_id = BASE64_STANDARD.encode(
            format!(
                "{}:{}",
                Uuid::new_v4(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            )
            .as_bytes(),
        );

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), Instant::now());

        session_id
    }

    pub async fn validate_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().await;

        if let Some(timestamp) = sessions.get_mut(session_id) {
            if timestamp.elapsed() < self.session_duration {
                // Session is valid, extend it
                *timestamp = Instant::now();
                true
            } else {
                // Session expired, remove it
                sessions.remove(session_id);
                false
            }
        } else {
            false
        }
    }

    pub async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    pub async fn cleanup_expired_sessions(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, timestamp| timestamp.elapsed() < self.session_duration);
    }
}
