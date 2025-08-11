use std::{path::PathBuf, time::SystemTime};

use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Clone)]
pub struct StatsData {
    pub win: u32,
    pub loss: u32,
}

#[derive(Resource)]
pub struct StatsFileWatcher {
    pub path: PathBuf,
    pub last_mtime: Option<SystemTime>,
    pub timer: Timer,
}

impl Default for StatsFileWatcher {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./stats.json"),
            last_mtime: None,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub win: u32,
    pub loss: u32,
}

pub async fn update_stats(Json(stats_payload): Json<StatsPayload>) -> impl IntoResponse {
    let stats_payload_str = match serde_json::to_string(&stats_payload) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to serialize auth payload: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error").into_response();
        }
    };

    if let Err(e) = std::fs::write("./stats.json", stats_payload_str) {
        error!("Failed to write auth file: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "File write error").into_response();
    }

    (StatusCode::OK, "OK").into_response()
}

pub fn poll_stats_file(
    time: Res<Time>,
    mut watcher: ResMut<StatsFileWatcher>,
    mut stats_data: ResMut<StatsData>,
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
                    if let Ok(payload) = serde_json::from_slice::<StatsPayload>(&bytes) {
                        stats_data.win = payload.loss;
                        stats_data.loss = payload.win;

                        info!("Stats OK -> win={}, loss={}", payload.win, payload.loss);

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
