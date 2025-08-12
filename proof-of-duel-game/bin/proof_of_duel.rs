use axum::{
    http::{HeaderValue, Method},
    routing::post,
};
use bevy::{
    audio::{AudioPlugin, SpatialScale},
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
// use bevy_fps_counter::FpsCounterPlugin;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_webserver::{BevyWebServerPlugin, RouterAppExt, WebServerConfig};
use proof_of_duel_game::{
    AUDIO_SCALE, GameState, LoggedInState, cameras,
    civic_auth::{self, AuthStateWatcher},
    connection::{self, ConnectionState, IsConnected},
    player::{self, PlayerHertsStatus, PlayerHit, PlayerSelection, PlayersCounting, ShootingLock},
    scene,
    shooting::{self, CheckShootingKeyEvent, ResetKeysEvent, ShootingEvent, ShootingStates},
    sounds,
    stats::{self, StatsData, StatsStateWatcher},
    ui::{
        self,
        game_over::WhoIsWinner,
        join_game::{BackspaceTimer, IsHost, MatchIdInput, MatchNotFoundError},
        main_menu::MainMenuState,
        play_now_ui::GameStartTimer,
        profile::ProfileData,
    },
};
use tower_http::cors::{Any, CorsLayer};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ShootingStates::default())
        .insert_resource(PlayerHertsStatus::default())
        .insert_resource(PlayerSelection::default())
        .insert_resource(GameStartTimer::new(3.0))
        .insert_resource(PlayersCounting::default())
        .insert_resource(WhoIsWinner::default())
        .insert_resource(ShootingLock::default())
        .insert_resource(MatchIdInput::default())
        .insert_resource(BackspaceTimer::default())
        .insert_resource(IsHost::default())
        .insert_resource(IsConnected::default())
        .insert_resource(ProfileData::default())
        .insert_resource(AuthStateWatcher::default())
        .insert_resource(StatsStateWatcher::default())
        .insert_resource(StatsData::default())
        .insert_resource(WebServerConfig {
            port: 8080,
            ..Default::default()
        })
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Proof of Duel".into(),
                    resizable: true,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 1280.,
                        min_height: 640.,
                        max_width: 1920.,
                        max_height: 1080.,
                    },
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AudioPlugin {
                default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                ..Default::default()
            })
            .set(LogPlugin {
                level: Level::ERROR,
                ..Default::default()
            }),))
        // .add_plugins(FpsCounterPlugin)
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(QuinnetClientPlugin::default())
        .add_plugins(BevyWebServerPlugin)
        .route("/login", post(civic_auth::login))
        .route("/update-stats", post(stats::update_stats))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers(Any)
                .expose_headers(Any)
                .allow_credentials(false),
        )
        .init_state::<GameState>()
        .init_state::<MainMenuState>()
        .init_state::<ConnectionState>()
        .init_state::<LoggedInState>()
        .add_event::<MatchNotFoundError>()
        .add_event::<ShootingEvent>()
        .add_event::<ResetKeysEvent>()
        .add_event::<CheckShootingKeyEvent>()
        .add_event::<PlayerHit>()
        .add_systems(Startup, ui::profile::spawn_profile_ui)
        .add_systems(
            Update,
            (
                civic_auth::poll_auth_state,
                stats::get_stats_scheduler,
                ui::profile::update_username,
                stats::poll_stats_state,
                stats::get_stats_scheduler,
                ui::profile::update_win,
                ui::profile::update_loss,
            ),
        )
        .add_systems(
            OnEnter(MainMenuState::MainMenu),
            (
                ui::join_game::reset_is_host,
                cameras::main_menu_camera_setup,
                connection::disconnect,
            ),
        )
        .add_systems(
            OnEnter(LoggedInState::NotLoggedIn),
            ui::main_menu::spawn_main_menu_before_logged_in,
        )
        .add_systems(
            OnExit(LoggedInState::NotLoggedIn),
            ui::main_menu::despawn_main_menu,
        )
        .add_systems(
            OnEnter(LoggedInState::LoggedIn),
            ui::main_menu::spawn_main_menu_after_logged_in,
        )
        .add_systems(
            OnExit(LoggedInState::LoggedIn),
            ui::main_menu::despawn_main_menu,
        )
        .add_systems(
            Update,
            (
                ui::main_menu::main_menu_button_pressed_handler,
                ui::main_menu::main_menu_ui_interaction,
            )
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                cameras::despawn_main_menu_camera,
                ui::main_menu::despawn_main_menu,
                cameras::despawn_play_now_ui_camera,
                ui::play_now_ui::despawn_play_now_ui,
            ),
        )
        .add_systems(
            OnExit(MainMenuState::MainMenu),
            (
                ui::main_menu::despawn_main_menu,
                cameras::despawn_main_menu_camera,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(MainMenuState::PlayNow),
            (
                cameras::play_now_ui_camera_setup,
                ui::play_now_ui::spawn_play_now_ui,
                connection::open_connection,
                connection::to_connection_state,
                ui::play_now_ui::create_room,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::play_now_ui::play_now_button_pressed_handler,
                ui::play_now_ui::play_now_ui_interaction,
                ui::play_now_ui::update_play_now_text,
                ui::play_now_ui::update_game_start_countdown,
            )
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::PlayNow)),
        )
        .add_systems(
            OnExit(MainMenuState::PlayNow),
            (
                cameras::despawn_play_now_ui_camera,
                ui::play_now_ui::despawn_play_now_ui,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(MainMenuState::JoinGame),
            (
                cameras::join_game_ui_camera_setup,
                ui::join_game::spawn_join_game_ui,
                connection::open_connection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::join_game::join_game_button_pressed_handler,
                ui::join_game::join_game_ui_interaction,
                ui::join_game::update_match_id_input,
                ui::join_game::match_not_found_error,
            )
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::JoinGame)),
        )
        .add_systems(
            OnExit(MainMenuState::JoinGame),
            (
                cameras::despawn_join_game_ui_camera,
                ui::join_game::despawn_join_game_ui,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                cameras::game_camera_setup,
                scene::setup_background,
                player::setup_player_1,
                player::setup_player_2,
                shooting::spawn_shooting_keys,
                sounds::music::play_bg_music,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                shooting::shooting_key_input,
                shooting::spawn_new_shooting_keys,
                player::player_shooting,
                player::who_was_hit,
                player::update_heart_status,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            connection::handle_server_messages.run_if(in_state(ConnectionState::Connected)),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                sounds::gun_shot::clear_gun_shot_sound,
                sounds::music::stop_playing_bg_music,
                scene::despawn_background,
                player::despawn_player,
                shooting::despawn_shooting_keys,
                cameras::despawn_game_camera,
            ),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (
                cameras::game_over_camera_setup,
                ui::game_over::spawn_game_over_ui,
                connection::reset_game_started_timer,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::game_over::game_over_button_pressed_handler,
                ui::game_over::game_over_ui_interaction,
            )
                .run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            (
                ui::game_over::despawn_game_over_ui,
                cameras::despawn_game_over_camera,
            ),
        )
        .run();
}
