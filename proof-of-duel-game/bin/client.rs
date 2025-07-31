use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_quinnet::client::QuinnetClientPlugin;
use proof_of_duel_game::{
    AUDIO_SCALE, GameState, cameras,
    connection::{self, ConnectionState},
    player::{
        self, CheckIsGameOverEvent, PlayerHertsStatus, PlayerHit, PlayerSelection, PlayersCounting,
    },
    scene,
    shooting::{self, CheckShootingKeyEvent, ResetKeysEvent, ShootingEvent, ShootingStates},
    sounds,
    ui::{
        self,
        main_menu::{GameStartTimer, MainMenuState},
    },
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ShootingStates::default())
        .insert_resource(PlayerHertsStatus::default())
        .insert_resource(PlayerSelection::default())
        .insert_resource(GameStartTimer::new(3.0))
        .insert_resource(PlayersCounting::default())
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
            }),))
        .add_plugins(FpsCounterPlugin)
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(QuinnetClientPlugin::default())
        .init_state::<GameState>()
        .init_state::<MainMenuState>()
        .init_state::<ConnectionState>()
        .add_event::<ShootingEvent>()
        .add_event::<ResetKeysEvent>()
        .add_event::<CheckShootingKeyEvent>()
        .add_event::<PlayerHit>()
        .add_event::<CheckIsGameOverEvent>()
        .add_systems(
            OnEnter(MainMenuState::MainMenu),
            (
                ui::main_menu::spawn_main_menu,
                cameras::main_menu_camera_setup,
            ),
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
                ui::main_menu::despawn_play_now_ui,
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
                ui::main_menu::spawn_play_now_ui,
                connection::open_connection,
                connection::to_connection_state,
            )
                .chain(),
        )
        .add_systems(
            Update,
            connection::handle_server_messages.run_if(in_state(ConnectionState::Connected)),
        )
        .add_systems(
            Update,
            (
                ui::main_menu::play_now_button_pressed_handler,
                ui::main_menu::play_now_ui_interaction,
                ui::main_menu::update_play_now_text,
                ui::main_menu::update_game_start_countdown,
            )
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::PlayNow)),
        )
        .add_systems(
            OnExit(MainMenuState::PlayNow),
            (
                cameras::despawn_play_now_ui_camera,
                ui::main_menu::despawn_play_now_ui,
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
                player::player_hearts_status_update,
                player::change_state_to_game_over,
            )
                .run_if(in_state(GameState::InGame)),
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
                connection::to_disconnected_state,
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
