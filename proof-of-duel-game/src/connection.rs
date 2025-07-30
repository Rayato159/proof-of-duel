use bevy::prelude::*;
use bevy_quinnet::{
    client::{
        QuinnetClient, certificate::CertificateVerificationMode,
        connection::ClientEndpointConfiguration,
    },
    shared::channels::{ChannelId, ChannelKind, ChannelsConfiguration},
};

use crate::{
    LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, ServerMessage,
    player::{PlayerSelection, PlayersCounting},
    ui::main_menu::GameStartTimer,
};
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    #[default]
    Idle,
    Connected,
}

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
                if is_ready && !game_start_timer.is_running {
                    game_start_timer.is_running = true;
                }
            }
        }
    }
}

pub fn to_connection_state(mut next_connection_state: ResMut<NextState<ConnectionState>>) {
    next_connection_state.set(ConnectionState::Connected);
}

pub fn to_disconnected_state(
    mut next_connection_state: ResMut<NextState<ConnectionState>>,
    mut client: ResMut<QuinnetClient>,
    mut player_selection: ResMut<PlayerSelection>,
) {
    client.close_connection(player_selection.1).unwrap();
    player_selection.reset();

    next_connection_state.set(ConnectionState::Idle);
}
