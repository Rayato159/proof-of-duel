use std::collections::HashMap;

use bevy::prelude::*;
use bevy_quinnet::{
    server::{
        ConnectionEvent, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration,
        certificate::CertificateRetrievalMode,
    },
    shared::{
        ClientId,
        channels::{ChannelId, ChannelKind, ChannelsConfiguration, DEFAULT_MAX_RELIABLE_FRAME_LEN},
    },
};
use proof_of_duel_game::{LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, ServerMessage, player::Player};

#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct Players {
    map: HashMap<ClientId, Player>,
}

#[repr(u8)]
pub enum ServerChannel {
    HostingLobby,
    InGame,
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
    for client_id in endpoint.clients() {}
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
            let player_number = players.map.len() + 1;

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
                .send_message(
                    client.id,
                    &ServerMessage::PlayerSelection {
                        player_number,
                        client_id: client.id,
                    },
                )
                .unwrap();

            if players.map.len() == 2 {
                server
                    .endpoint_mut()
                    .broadcast_message(&ServerMessage::IsGameReadyToStart { is_ready: true })
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
