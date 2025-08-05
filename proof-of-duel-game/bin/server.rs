use std::collections::HashMap;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_quinnet::{
    server::{
        ConnectionLostEvent, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration,
        certificate::CertificateRetrievalMode,
    },
    shared::ClientId,
};
use proof_of_duel_game::{
    ClientMessage, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, ServerChannel, ServerMessage,
    player::Player,
};
use uuid::Uuid;

#[derive(Resource, Default, Debug)]
pub struct Matches {
    pub sessions: HashMap<Uuid, MatchSession>,
}

#[derive(Debug)]
pub struct MatchSession {
    pub id: Uuid,
    pub players: HashMap<ClientId, Player>,
    pub player_1_hearts: usize,
    pub player_2_hearts: usize,
}

impl MatchSession {
    pub fn is_full(&self) -> bool {
        self.players.len() == 2
    }
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(LOCAL_BIND_IP, SERVER_PORT),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: SERVER_HOST.to_string(),
            },
            ServerChannel::channels_configuration(),
        )
        .unwrap();
}

fn handle_client_messages(mut server: ResMut<QuinnetServer>, mut matches: ResMut<Matches>) {
    let endpoint = server.endpoint_mut();

    for client_id in endpoint.clients() {
        while let Some((channel_id, message)) =
            endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::CreateMatchRequest {
                    match_id,
                    player_wallet,
                } => {
                    if channel_id == 0 {
                        let player_1_hearts = 5;
                        let player_2_hearts = 5;

                        let mut new_session = MatchSession {
                            id: match_id,
                            players: HashMap::new(),
                            player_1_hearts,
                            player_2_hearts,
                        };

                        new_session.players.insert(
                            client_id,
                            Player {
                                player_number: 1,
                                wallet: player_wallet,
                            },
                        );

                        matches.sessions.insert(match_id, new_session);

                        endpoint
                            .send_message_on(
                                client_id,
                                ServerChannel::Lobby,
                                &ServerMessage::MatchCreated {
                                    match_id: match_id,
                                    player_number: 1,
                                },
                            )
                            .unwrap();

                        endpoint
                            .send_message_on(
                                client_id,
                                ServerChannel::Lobby,
                                &ServerMessage::PlayerCountingUpdate { match_id },
                            )
                            .unwrap();
                    }
                }

                ClientMessage::JoinMatchRequest {
                    match_id,
                    player_wallet,
                } => {
                    if channel_id == 0 {
                        match matches.sessions.get_mut(&match_id) {
                            Some(session) => {
                                if session.is_full() {
                                    endpoint
                                        .send_message_on(
                                            client_id,
                                            ServerChannel::Lobby,
                                            &ServerMessage::MatchJoinError {
                                                error_message: "Match is full".to_string(),
                                            },
                                        )
                                        .unwrap();
                                } else {
                                    let player_number: usize = 2;

                                    session.players.insert(
                                        client_id,
                                        Player {
                                            player_number,
                                            wallet: player_wallet,
                                        },
                                    );

                                    endpoint
                                        .send_message_on(
                                            client_id,
                                            ServerChannel::Lobby,
                                            &ServerMessage::JoinedMatch {
                                                match_id,
                                                player_number,
                                            },
                                        )
                                        .unwrap();

                                    endpoint
                                        .send_message_on(
                                            client_id,
                                            ServerChannel::Lobby,
                                            &ServerMessage::PlayerCountingUpdate { match_id },
                                        )
                                        .unwrap();

                                    if session.is_full() {
                                        for pid in session.players.keys() {
                                            endpoint
                                                .send_message_on(
                                                    *pid,
                                                    ServerChannel::Lobby,
                                                    &ServerMessage::IsGameReadyToStart {
                                                        match_id,
                                                        is_ready: true,
                                                    },
                                                )
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                            None => {
                                endpoint
                                    .send_message_on(
                                        client_id,
                                        ServerChannel::Lobby,
                                        &ServerMessage::MatchJoinError {
                                            error_message: "Match not found".to_string(),
                                        },
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
                ClientMessage::ShootingCommand {
                    match_id,
                    player_number,
                } => {
                    if let Some(session) = matches.sessions.get_mut(&match_id) {
                        if session.players.get(&client_id).is_some() && channel_id == 1 {
                            if player_number == 1 {
                                session.player_2_hearts = session.player_2_hearts.saturating_sub(1);
                            } else {
                                session.player_1_hearts = session.player_1_hearts.saturating_sub(1);
                            }

                            endpoint
                                .broadcast_message_on(
                                    ServerChannel::UpdateHeartsStatus,
                                    &ServerMessage::UpdateHeartsStatus {
                                        match_id,
                                        who_is_hit: player_number,
                                        player_1_hearts: session.player_1_hearts,
                                        player_2_hearts: session.player_2_hearts,
                                    },
                                )
                                .unwrap();

                            endpoint
                                .broadcast_message_on(
                                    ServerChannel::Shooting,
                                    &ServerMessage::ShootingCommand {
                                        match_id,
                                        player_number,
                                    },
                                )
                                .unwrap();

                            if session.player_1_hearts == 0 || session.player_2_hearts == 0 {
                                let winner = if session.player_1_hearts == 0
                                    && session.player_2_hearts == 0
                                {
                                    0
                                } else if session.player_1_hearts == 0 {
                                    2
                                } else {
                                    1
                                };

                                endpoint
                                    .broadcast_message_on(
                                        ServerChannel::GameOver,
                                        &ServerMessage::GameOver { match_id, winner },
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_disconnection(
    mut disconnect_events: EventReader<ConnectionLostEvent>,
    mut matches: ResMut<Matches>,
) {
    for event in disconnect_events.read() {
        let client_id = event.id;

        let mut session_to_remove: Option<Uuid> = None;

        for (id, session) in matches.sessions.iter_mut() {
            if session.players.remove(&client_id).is_some() {
                println!("Client {:?} disconnected from match {:?}", client_id, id);

                if session.players.len() < 2 {
                    println!("Match {:?} is no longer active. Removing it.", id);
                    session_to_remove = Some(*id);
                }
                break;
            }
        }

        if let Some(id) = session_to_remove {
            matches.sessions.remove(&id);
        }
    }
}

pub fn main() {
    App::new()
        .insert_resource(Matches::default())
        .add_plugins(ScheduleRunnerPlugin::default())
        .add_plugins(QuinnetServerPlugin::default())
        .add_systems(Startup, start_listening)
        .add_systems(Update, handle_client_messages)
        .add_systems(Update, handle_disconnection)
        .run();
}
