use std::sync::{Arc, LazyLock, RwLock};

use axum::{Json, http::StatusCode, response::IntoResponse};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{LoggedInState, ui::profile::ProfileData};

#[derive(Default)]
struct AuthState {
    public_key: String,
    username: String,
}

static AUTH_STATE: LazyLock<Arc<RwLock<AuthState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(AuthState::default())));

#[derive(Resource)]
pub struct AuthStateWatcher {
    pub timer: Timer,
}

impl Default for AuthStateWatcher {
    fn default() -> Self {
        Self {
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
    let mut auth_state = AUTH_STATE.write().unwrap();
    auth_state.public_key = auth_payload.public_key.clone();
    auth_state.username = auth_payload.username.clone();

    (StatusCode::OK, "OK").into_response()
}

pub fn poll_auth_state(
    time: Res<Time>,
    mut watcher: ResMut<AuthStateWatcher>,
    mut profile_data: ResMut<ProfileData>,
    mut next_logged_in_sate: ResMut<NextState<LoggedInState>>,
) {
    if profile_data.logged_in {
        return;
    }

    if !watcher.timer.tick(time.delta()).finished() {
        return;
    }

    let auth_state = AUTH_STATE.read().unwrap();

    if !auth_state.public_key.is_empty() && !auth_state.username.is_empty() {
        profile_data.logged_in = true;
        profile_data.public_key = auth_state.public_key.clone();
        profile_data.username = auth_state.username.clone();

        next_logged_in_sate.set(LoggedInState::LoggedIn);
    } else {
        profile_data.logged_in = false;
        profile_data.public_key.clear();
        profile_data.username.clear();
    }
}
