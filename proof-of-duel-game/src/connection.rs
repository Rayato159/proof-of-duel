use bevy::prelude::*;
use bevy_quinnet::client::{
    QuinnetClient, certificate::CertificateVerificationMode,
    connection::ClientEndpointConfiguration,
};

use crate::{
    ClientChannel, GameState, LOCAL_BIND_IP, SERVER_PORT, ServerMessage, get_ip,
    player::{PlayerHertsStatus, PlayerHit, PlayerSelection, PlayersCounting},
    shooting::ShootingEvent,
    ui::{game_over::WhoIsWinner, join_game::MatchNotFoundError, play_now_ui::GameStartTimer},
};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    #[default]
    Idle,
    Connected,
}

#[derive(Resource, Default)]
pub struct IsConnected(pub bool);

impl IsConnected {
    pub fn reset(&mut self) {
        self.0 = false;
    }
}

pub fn disconnect(mut is_connected: ResMut<IsConnected>) {
    is_connected.reset();
}

pub fn open_connection(mut client: ResMut<QuinnetClient>, mut is_connected: ResMut<IsConnected>) {
    if is_connected.0 {
        return;
    }

    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(get_ip().unwrap(), SERVER_PORT, LOCAL_BIND_IP, 0),
            CertificateVerificationMode::SkipVerification,
            ClientChannel::channels_configuration(),
        )
        .unwrap();

    is_connected.0 = true;
}

pub fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut player_selection: ResMut<PlayerSelection>,
    mut players_counting: ResMut<PlayersCounting>,
    mut game_start_timer: ResMut<GameStartTimer>,
    mut player_hearts_status: ResMut<PlayerHertsStatus>,
    mut shooting_event: EventWriter<ShootingEvent>,
    mut player_hit: EventWriter<PlayerHit>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut who_is_winner: ResMut<WhoIsWinner>,
    mut match_not_found_error_event: EventWriter<MatchNotFoundError>,
) {
    while let Some((channel, message)) = client
        .connection_mut()
        .try_receive_message::<ServerMessage>()
    {
        match message {
            ServerMessage::MatchCreated {
                player_number,
                match_id,
            } => {
                if channel == 0 {
                    player_selection.0 = player_number;
                    player_selection.1 = match_id;
                }
            }
            ServerMessage::JoinedMatch {
                player_number,
                match_id,
            } => {
                if channel == 0 {
                    player_selection.0 = player_number;
                    player_selection.1 = match_id;
                }
            }
            ServerMessage::PlayerCountingUpdate { match_id } => {
                if channel == 0 && match_id == player_selection.1 {
                    players_counting.0 += 1;
                }
            }
            ServerMessage::MatchJoinError { error_message } => {
                if channel == 0 {
                    match_not_found_error_event.write(MatchNotFoundError::new(error_message));
                }
            }
            ServerMessage::IsGameReadyToStart { match_id, is_ready } => {
                if is_ready
                    && !game_start_timer.active
                    && channel == 0
                    && match_id == player_selection.1
                {
                    game_start_timer.active = true;
                }
            }
            ServerMessage::ShootingCommand {
                match_id,
                player_number,
            } => {
                if channel == 1 && match_id == player_selection.1 {
                    shooting_event.write(ShootingEvent {
                        player: player_number,
                    });
                }
            }
            ServerMessage::UpdateHeartsStatus {
                match_id,
                who_is_hit,
                player_1_hearts,
                player_2_hearts,
            } => {
                if channel == 2 && match_id == player_selection.1 {
                    let who_was_hit = if who_is_hit == 1 { 2 } else { 1 };
                    player_hit.write(PlayerHit(who_was_hit));

                    player_hearts_status.player_1_hearts = player_1_hearts;
                    player_hearts_status.player_2_hearts = player_2_hearts;
                }
            }
            ServerMessage::GameOver { match_id, winner } => {
                if channel == 3 && match_id == player_selection.1 {
                    who_is_winner.player_number = winner;
                    next_game_state.set(GameState::GameOver);
                }
            }
        }
    }
}

pub fn to_connection_state(mut next_connection_state: ResMut<NextState<ConnectionState>>) {
    next_connection_state.set(ConnectionState::Connected);
}

pub fn reset_game_started_timer(mut game_start_timer: ResMut<GameStartTimer>) {
    game_start_timer.reset();
}
