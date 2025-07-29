use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use proof_of_duel_game::{
    AUDIO_SCALE, GameState, cameras,
    player::{
        self, CheckIsGameOverEvent, ChoosePlayerEvent, PlayerHertsStatus, PlayerHit,
        PlayerSelection,
    },
    scene,
    shooting::{
        self, CheckShootingKeyEvent, DuelRound, ResetKeysEvent, ShootingEvent,
        ShootingStatesContainer,
    },
    sounds,
    ui::{self, main_menu::MainMenuState},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DuelRound::default())
        .insert_resource(ShootingStatesContainer::default())
        .insert_resource(PlayerHertsStatus::default())
        .insert_resource(PlayerSelection::default())
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
        .init_state::<GameState>()
        .init_state::<MainMenuState>()
        .add_event::<ChoosePlayerEvent>()
        .add_event::<ShootingEvent>()
        .add_event::<ResetKeysEvent>()
        .add_event::<CheckShootingKeyEvent>()
        .add_event::<PlayerHit>()
        .add_event::<CheckIsGameOverEvent>()
        .add_systems(
            OnEnter(GameState::MainMenu),
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
                .run_if(in_state(MainMenuState::None)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                ui::main_menu::despawn_main_menu,
                cameras::despawn_main_menu_camera,
            ),
        )
        .add_systems(
            OnExit(MainMenuState::None),
            (
                ui::main_menu::despawn_main_menu,
                cameras::despawn_main_menu_camera,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(MainMenuState::PlayGame),
            (
                cameras::main_menu_play_game_ui_camera_setup,
                ui::main_menu::spawn_play_game_ui,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                ui::main_menu::play_game_button_pressed_handler,
                ui::main_menu::play_game_ui_interaction,
                player::choose_your_player,
            )
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::PlayGame)),
        )
        .add_systems(
            OnExit(MainMenuState::PlayGame),
            (
                cameras::despawn_main_menu_play_game_ui_camera,
                ui::main_menu::despawn_play_game_ui,
                cameras::main_menu_camera_setup,
                ui::main_menu::spawn_main_menu,
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
                player::player_1_shooting,
                player::player_2_shooting,
                player::player_2_hearts_status_update,
                player::player_1_hearts_status_update,
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
                player::despawn_player_1,
                player::despawn_player_2,
                shooting::despawn_shooting_keys,
                cameras::despawn_game_camera,
            ),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (
                cameras::game_over_camera_setup,
                ui::game_over::spawn_game_over_ui,
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
