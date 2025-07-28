use bevy::{
    audio::{AudioPlugin, SpatialScale},
    prelude::*,
};
use bevy_aseprite_ultra::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use proof_of_duel_game::{
    AUDIO_SCALE, GameState, cameras,
    player::{self, CheckIsGameOverEvent, PlayerHertsStatus, PlayerHit},
    scene,
    shooting::{
        self, CheckShootingKeyEvent, DuelRound, ResetKeysEvent, ShootingEvent,
        ShootingStatesContainer,
    },
    sounds,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DuelRound::default())
        .insert_resource(ShootingStatesContainer::default())
        .insert_resource(PlayerHertsStatus::default())
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
        .add_event::<ShootingEvent>()
        .add_event::<ResetKeysEvent>()
        .add_event::<CheckShootingKeyEvent>()
        .add_event::<PlayerHit>()
        .add_event::<CheckIsGameOverEvent>()
        .add_systems(OnEnter(GameState::InGame), cameras::game_camera_setup)
        .add_systems(OnEnter(GameState::InGame), scene::setup_background)
        .add_systems(OnEnter(GameState::InGame), player::setup_player_1)
        .add_systems(OnEnter(GameState::InGame), player::setup_player_2)
        .add_systems(OnEnter(GameState::InGame), shooting::spawn_shooting_keys)
        .add_systems(OnEnter(GameState::InGame), sounds::music::play_bg_music)
        .add_systems(
            Update,
            (
                shooting::shooting_key_input,
                shooting::spawn_new_shooting_keys,
                player::player_1_shooting,
                player::player_2_hearts_status_update,
                player::player_1_hearts_status_update,
                player::change_state_to_game_over,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
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
            cameras::game_over_camera_setup,
        )
        .add_systems(OnEnter(GameState::GameOver), scene::check_who_is_winner)
        .add_systems(
            OnExit(GameState::GameOver),
            (
                scene::despawn_game_over_ui,
                cameras::despawn_game_over_camera,
            ),
        )
        .run();
}
