use std::{path::PathBuf, time::SystemTime};

use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::profile::ProfileData;

#[derive(Resource)]
pub struct AuthFileWatcher {
    pub path: PathBuf,
    pub last_mtime: Option<SystemTime>,
    pub timer: Timer,
}

impl Default for AuthFileWatcher {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./auth.json"),
            last_mtime: None,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthPayload {
    public_key: String,
    username: String,
}

pub async fn login(Json(auth_payload): Json<AuthPayload>) -> impl IntoResponse {
    let auth_payload_str = match serde_json::to_string(&auth_payload) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to serialize auth payload: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error").into_response();
        }
    };

    if let Err(e) = std::fs::write("./auth.json", auth_payload_str) {
        error!("Failed to write auth file: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "File write error").into_response();
    }

    (StatusCode::OK, "OK").into_response()
}

pub fn poll_auth_file(
    time: Res<Time>,
    mut watcher: ResMut<AuthFileWatcher>,
    mut profile_data: ResMut<ProfileData>,
) {
    if !watcher.timer.tick(time.delta()).finished() {
        return;
    }

    let path = watcher.path.clone();

    if !path.exists() {
        if let Err(e) = std::fs::write(&path, "") {
            error!("Failed to create auth file: {}", e);
        } else {
            info!("Created empty auth file at {:?}", path);
        }
    }

    if let Ok(meta) = std::fs::metadata(&path) {
        if let Ok(mtime) = meta.modified() {
            let should_read = watcher.last_mtime.map(|t| t != mtime).unwrap_or(true);

            if should_read {
                if let Ok(bytes) = std::fs::read(&path) {
                    if let Ok(payload) = serde_json::from_slice::<AuthPayload>(&bytes) {
                        profile_data.logged_in = true;
                        profile_data.public_key = payload.public_key;
                        profile_data.username = payload.username;

                        info!(
                            "Auth OK -> user={:?}, pk={:?}",
                            profile_data.username, profile_data.public_key
                        );

                        let _ = std::fs::remove_file(&path);
                        watcher.last_mtime = None;

                        return;
                    }
                }

                watcher.last_mtime = Some(mtime);
            }
        }
    }
}
