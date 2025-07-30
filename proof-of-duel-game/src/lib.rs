use std::net::Ipv4Addr;

use bevy::prelude::*;

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

pub mod cameras;
pub mod connection;
pub mod player;
pub mod scene;
pub mod shooting;
pub mod sounds;
pub mod ui;
