use bevy::prelude::*;
use bevy_quinnet::{
    client::{
        QuinnetClient, certificate::CertificateVerificationMode,
        connection::ClientEndpointConfiguration,
    },
    shared::channels::{ChannelId, ChannelKind, ChannelsConfiguration},
};
use serde::{Deserialize, Serialize};

use crate::{
    LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT,
    player::{PlayerSelection, PlayersCounting},
    ui::main_menu::GameStartTimer,
};

#[repr(u8)]
pub enum ClientChannel {
    ShootingCommand,
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
pub enum ServerMessage {
    PlayerSelection {
        player_number: usize,
        client_id: u64,
    },
    IsGameReadyToStart {
        is_ready: bool,
    },
}

pub fn open_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(SERVER_HOST, SERVER_PORT, LOCAL_BIND_IP, 0),
            CertificateVerificationMode::SkipVerification,
            ClientChannel::channels_configuration(),
        )
        .unwrap();
}

pub fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut player_selection: ResMut<PlayerSelection>,
    mut players_counting: ResMut<PlayersCounting>,
    mut game_start_timer: ResMut<GameStartTimer>,
) {
    while let Some((_, message)) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::PlayerSelection {
                player_number,
                client_id,
            } => {
                player_selection.0 = player_number;
                player_selection.1 = client_id;

                players_counting.0 += 1;
            }
            ServerMessage::IsGameReadyToStart { is_ready } => {
                if is_ready {
                    game_start_timer.is_running = true;
                }
            }
        }
    }
}
