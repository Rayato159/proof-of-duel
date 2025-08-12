use std::sync::{Arc, LazyLock, RwLock};

use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use serde::{Deserialize, Serialize};

use crate::ui::profile::{DuelInfoPayload, ProfileData};

#[derive(Default)]
struct StatsState {
    pub win: u32,
    pub loss: u32,
}

static STATS_STATE: LazyLock<Arc<RwLock<StatsState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(StatsState::default())));

#[derive(Resource, Default, Clone)]
pub struct StatsData {
    pub win: u32,
    pub loss: u32,
}

#[derive(Resource)]
pub struct StatsStateWatcher {
    pub timer: Timer,
}

impl Default for StatsStateWatcher {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatsPayload {
    pub win: u32,
    pub loss: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub win: u32,
    pub loss: u32,
}

pub async fn update_stats(Json(stats_payload): Json<StatsPayload>) -> impl IntoResponse {
    let mut stats_state = STATS_STATE.write().unwrap();
    stats_state.win = stats_payload.win;
    stats_state.loss = stats_payload.loss;

    (StatusCode::OK, "OK").into_response()
}

pub fn get_stats_scheduler(
    time: Res<Time>,
    mut watcher: ResMut<StatsStateWatcher>,
    profile_data: Res<ProfileData>,
) {
    if !profile_data.logged_in {
        return;
    }

    if !watcher.timer.tick(time.delta()).finished() {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();
    let public_key = profile_data.public_key.clone();

    thread_pool
        .spawn(async move {
            let url = "http://localhost:3000/api/duel-info";
            match ureq::post(url).send_json(DuelInfoPayload { public_key }) {
                Ok(response) if response.status() == 200 => {
                    info!("✅ Duel loss recorded successfully");
                }
                Ok(response) => {
                    error!("❌ Duel loss failed to record: {}", response.status());
                }
                Err(e) => {
                    error!("❌ Error sending to RPC: {:?}", e);
                }
            }
        })
        .detach();
}

pub fn poll_stats_state(
    time: Res<Time>,
    mut watcher: ResMut<StatsStateWatcher>,
    mut stats_data: ResMut<StatsData>,
    profile_data: Res<ProfileData>,
) {
    if !profile_data.logged_in {
        return;
    }

    if !watcher.timer.tick(time.delta()).finished() {
        return;
    }

    let stats_state = STATS_STATE.read().unwrap();

    if stats_state.win != stats_data.win || stats_state.loss != stats_data.loss {
        stats_data.win = stats_state.win;
        stats_data.loss = stats_state.loss;
    }
}
