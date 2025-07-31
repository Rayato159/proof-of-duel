use bevy::prelude::*;
use bevy_quinnet::client::{
    QuinnetClient, certificate::CertificateVerificationMode,
    connection::ClientEndpointConfiguration,
};

use crate::{
    ClientChannel, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, ServerMessage,
    player::{PlayerHertsStatus, PlayerSelection, PlayersCounting},
    shooting::ShootingEvent,
    ui::main_menu::GameStartTimer,
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    #[default]
    Idle,
    Connected,
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
    mut player_herts_status: ResMut<PlayerHertsStatus>,
    mut shooting_event: EventWriter<ShootingEvent>,
) {
    while let Some((channel, message)) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::PlayerSelection {
                player_number,
                client_id,
            } => {
                if channel == 0 {
                    player_selection.0 = player_number;
                    player_selection.1 = client_id;

                    players_counting.0 += 1;
                }
            }
            ServerMessage::IsGameReadyToStart { is_ready } => {
                if is_ready && !game_start_timer.active && channel == 0 {
                    game_start_timer.active = true;
                }
            }
            ServerMessage::ShootingCommand {
                player_number,
                states,
            } => {
                if channel == 1 {
                    shooting_event.write(ShootingEvent {
                        player: player_number,
                        states,
                    });
                }
            }
            ServerMessage::PlayerHeartsStatus {
                player_1_hearts,
                player_2_hearts,
            } => {
                player_herts_status.player_1_hearts = player_1_hearts;
                player_herts_status.player_2_hearts = player_2_hearts;
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

pub fn reset_game_started_timer(mut game_start_timer: ResMut<GameStartTimer>) {
    game_start_timer.reset();
}
