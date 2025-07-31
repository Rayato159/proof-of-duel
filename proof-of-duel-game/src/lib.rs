use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy_quinnet::shared::channels::{
    ChannelId, ChannelKind, ChannelsConfiguration, DEFAULT_MAX_RELIABLE_FRAME_LEN,
};
use serde::{Deserialize, Serialize};

pub(crate) const GRID_SIZE: f32 = 32.0;
pub(crate) const MAP_SIZE_X: usize = 40;
pub(crate) const MUSIC_VOLUME: f32 = 0.8;
pub const AUDIO_SCALE: f32 = 1. / 100.0;

pub const SERVER_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 6000;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
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
            ChannelKind::UnorderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::Unreliable,
        ])
        .unwrap()
    }
}

#[repr(u8)]
pub enum ClientChannel {
    Lobby,
    Shooting,
    UpdateHeartsStatus,
    GameOver,
}

impl Into<ChannelId> for ClientChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}

impl ClientChannel {
    pub fn channels_configuration() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![ChannelKind::default()]).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShootingStatesMesasge {
    pub key: String,
    pub is_pressed_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerSelection {
        player_number: usize,
        client_id: u64,
    },
    IsGameReadyToStart {
        is_ready: bool,
    },
    ShootingCommand {
        player_number: usize,
        states: [ShootingStatesMesasge; 5],
    },
    UpdateHeartsStatus {
        player_1_hearts: usize,
        player_2_hearts: usize,
        who_was_hit: usize,
    },
    GameOver {
        winner: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    ShootingCommand {
        player_number: usize,
        states: [ShootingStatesMesasge; 5],
    },
    UpdateHeartsStatus {
        player_1_hearts: usize,
        player_2_hearts: usize,
        who_was_hit: usize,
    },
    GameOver {
        winner: usize,
    },
    DisconnectPlayer {
        client_id: u64,
    },
}

pub mod cameras;
pub mod connection;
pub mod player;
pub mod scene;
pub mod shooting;
pub mod sounds;
pub mod ui;
