use std::collections::HashMap;

use bevy::prelude::*;
use bevy_quinnet::{
    server::{
        ConnectionEvent, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration,
        certificate::CertificateRetrievalMode,
    },
    shared::ClientId,
};
use proof_of_duel_game::{
    ClientMessage, LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, ServerChannel, ServerMessage,
    player::Player,
};

#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct Players {
    map: HashMap<ClientId, Player>,
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

fn handle_client_messages(mut server: ResMut<QuinnetServer>, mut players: ResMut<Players>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some((_, message)) = endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::ShootingCommand {
                    player_number,
                    states,
                } => {
                    if players.map.get(&client_id).is_some() {
                        endpoint
                            .broadcast_message_on(
                                ServerChannel::Shooting,
                                &ServerMessage::ShootingCommand {
                                    player_number,
                                    states,
                                },
                            )
                            .unwrap();
                    }
                }
                ClientMessage::UpdateHeartsStatus {
                    player_1_hearts,
                    player_2_hearts,
                    who_was_hit,
                } => {
                    if players.map.get(&client_id).is_some() {
                        if who_was_hit == 1 {
                            endpoint
                                .broadcast_message_on(
                                    ServerChannel::UpdateHeartsStatus,
                                    &ServerMessage::UpdateHeartsStatus {
                                        player_1_hearts: player_1_hearts.saturating_sub(1),
                                        player_2_hearts,
                                        who_was_hit,
                                    },
                                )
                                .unwrap();
                        } else if who_was_hit == 2 {
                            endpoint
                                .broadcast_message_on(
                                    ServerChannel::UpdateHeartsStatus,
                                    &ServerMessage::UpdateHeartsStatus {
                                        player_1_hearts,
                                        player_2_hearts: player_2_hearts.saturating_sub(1),
                                        who_was_hit,
                                    },
                                )
                                .unwrap();
                        }
                    }
                }
                ClientMessage::GameOver { winner } => {
                    if players.map.get(&client_id).is_some() {
                        endpoint
                            .broadcast_message(&ServerMessage::GameOver { winner })
                            .unwrap();
                    }
                }
                ClientMessage::DisconnectPlayer { client_id } => {
                    if players.map.contains_key(&client_id) {
                        for client_id in endpoint.clients() {
                            let _ = endpoint.disconnect_client(client_id);
                            players.map.remove(&client_id);
                        }
                    }
                }
            }
        }
    }
}

fn handle_server_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut server: ResMut<QuinnetServer>,
    mut players: ResMut<Players>,
) {
    // The server signals us about new connections
    for client in connection_events.read() {
        // Refuse connection once we already have two players
        if players.map.len() >= 2 {
            server.endpoint_mut().disconnect_client(client.id).unwrap();
        } else {
            let player_number = if !players.map.values().any(|p| p.player_number == 1) {
                1
            } else {
                2
            };

            players.map.insert(
                client.id,
                Player {
                    client_id: client.id,
                    player_number,
                    wallet: "".to_string(),
                },
            );

            server
                .endpoint_mut()
                .send_message_on(
                    client.id,
                    ServerChannel::Lobby,
                    &ServerMessage::PlayerSelection {
                        player_number,
                        client_id: client.id,
                    },
                )
                .unwrap();

            if players.map.len() == 2 {
                server
                    .endpoint_mut()
                    .broadcast_message_on(
                        ServerChannel::Lobby,
                        &ServerMessage::IsGameReadyToStart { is_ready: true },
                    )
                    .unwrap();
            }
        }
    }
}

pub fn main() {
    App::new()
        .insert_resource(Players::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(QuinnetServerPlugin::default())
        .add_systems(Startup, start_listening)
        .add_systems(Update, handle_server_events)
        .add_systems(Update, handle_client_messages)
        .run();
}
