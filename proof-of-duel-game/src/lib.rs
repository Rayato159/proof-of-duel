use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_quinnet::shared::channels::{
    ChannelId, ChannelKind, ChannelsConfiguration, DEFAULT_MAX_RELIABLE_FRAME_LEN,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) const GRID_SIZE: f32 = 32.0;
pub(crate) const MAP_SIZE_X: usize = 40;
pub(crate) const MUSIC_VOLUME: f32 = 0.8;
pub const AUDIO_SCALE: f32 = 1. / 100.0;

pub const SERVER_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 6000;

pub fn get_ip() -> Result<Ipv4Addr> {
    dotenvy::dotenv().ok();

    let client_url = dotenvy::var("IP_ADDRESS")
        .expect("IP_ADDRESS is invalid")
        .parse()
        .expect("IP_ADDRESS is not a valid IP address");

    Ok(client_url)
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoggedInState {
    #[default]
    NotLoggedIn,
    LoggedIn,
    InGame,
}

#[repr(u8)]
pub enum ServerChannel {
    Lobby,
    Shooting,
    UpdateHeartsStatus,
    GameOver,
}

impl Into<ChannelId> for ServerChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}

impl ServerChannel {
    pub fn channels_configuration() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
        ])
        .unwrap()
    }
}

#[repr(u8)]
pub enum ClientChannel {
    Lobby,
    Shooting,
}

impl Into<ChannelId> for ClientChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}

impl ClientChannel {
    pub fn channels_configuration() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
        ])
        .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShootingStatesMesasge {
    pub key: String,
    pub is_pressed_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    MatchCreated {
        match_id: Uuid,
        player_number: usize,
    },
    JoinedMatch {
        match_id: Uuid,
        player_number: usize,
    },
    PlayerCountingUpdate {
        match_id: Uuid,
    },
    MatchJoinError {
        error_message: String,
    },
    IsGameReadyToStart {
        match_id: Uuid,
        is_ready: bool,
    },
    ShootingCommand {
        match_id: Uuid,
        player_number: usize,
    },
    UpdateHeartsStatus {
        match_id: Uuid,
        who_is_hit: usize,
        player_1_hearts: usize,
        player_2_hearts: usize,
    },
    GameOver {
        match_id: Uuid,
        winner: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateMatchRequest {
        match_id: Uuid,
        player_wallet: String,
    },
    JoinMatchRequest {
        match_id: Uuid,
        player_wallet: String,
    },
    ShootingCommand {
        match_id: Uuid,
        player_number: usize,
    },
}

pub mod cameras;
pub mod civic_auth;
pub mod connection;
pub mod player;
pub mod scene;
pub mod shooting;
pub mod sounds;
pub mod stats;
pub mod ui;
