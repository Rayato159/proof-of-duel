use bevy::{audio::Volume, prelude::*};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    GRID_SIZE, GameState, MAP_SIZE_X,
    shooting::{ResetKeysEvent, ShootingEvent},
    sounds::gun_shot::GunShotSound,
};

#[derive(Resource, Default)]
pub struct PlayerSelection(pub usize, pub u64);

impl PlayerSelection {
    pub fn reset(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }
}

#[derive(Resource, Default)]
pub struct PlayersCounting(pub usize);

#[derive(Debug, Clone, Component)]
pub struct Player {
    pub client_id: u64,
    pub player_number: usize,
    pub wallet: String,
}

impl Player {
    pub fn new(client_id: u64, player_number: usize, wallet: String) -> Self {
        Player {
            client_id,
            player_number,
            wallet,
        }
    }
}

#[derive(Resource)]
pub struct PlayerHertsStatus {
    pub player_1_hearts: usize,
    pub player_2_hearts: usize,
}

impl Default for PlayerHertsStatus {
    fn default() -> Self {
        Self {
            player_1_hearts: 5,
            player_2_hearts: 5,
        }
    }
}

#[derive(Component, Clone)]
pub struct PlayterHeart(pub usize, pub usize);

#[derive(Event)]
pub struct PlayerHit(pub usize);

#[derive(Event)]
pub struct CheckIsGameOverEvent;

pub fn setup_player_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_selection: Res<PlayerSelection>,
) {
    let aseprite = asset_server.load("sprites/Player1.aseprite");

    commands
        .spawn((
            Player::new(player_selection.1, 1, "".to_string()),
            AseAnimation {
                aseprite,
                animation: Animation::tag("idle").with_speed(1.),
            },
            Sprite::default(),
            Transform::from_xyz(
                -((GRID_SIZE * MAP_SIZE_X as f32) / 2. - (GRID_SIZE * 5.)),
                -(GRID_SIZE * 5.),
                100.,
            ),
        ))
        .with_children(|parent| {
            for i in 0..5 {
                parent.spawn((
                    PlayterHeart(1, i),
                    AseAnimation {
                        aseprite: asset_server.load("sprites/Heart.aseprite"),
                        animation: Animation::tag("Full").with_speed(1.),
                    },
                    Sprite::default(),
                    Transform::from_xyz(-GRID_SIZE * 2.5 + (i as f32 * 32.), GRID_SIZE * 3., 100.),
                ));
            }
        })
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Player 1"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font: asset_server.load("fonts/pixeloid_mono.ttf"),
                    font_size: 28.,
                    ..Default::default()
                },
                Transform::from_xyz(-GRID_SIZE * 0.5, GRID_SIZE * 4.5, 100.0),
            ));
        });
}

pub fn setup_player_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_selection: Res<PlayerSelection>,
) {
    let aseprite = asset_server.load("sprites/Player2.aseprite");

    commands
        .spawn((
            Player::new(player_selection.1, 2, "".to_string()),
            AseAnimation {
                aseprite,
                animation: Animation::tag("idle").with_speed(1.),
            },
            Sprite::default(),
            Transform::from_xyz(
                (GRID_SIZE * MAP_SIZE_X as f32) / 2. - (GRID_SIZE * 5.),
                -(GRID_SIZE * 5.),
                100.,
            ),
        ))
        .with_children(|parent| {
            for i in 0..5 {
                parent.spawn((
                    PlayterHeart(2, i),
                    AseAnimation {
                        aseprite: asset_server.load("sprites/Heart.aseprite"),
                        animation: Animation::tag("Full").with_speed(1.),
                    },
                    Sprite::default(),
                    Transform::from_xyz(GRID_SIZE * 2.5 - (i as f32 * 32.), GRID_SIZE * 3., 100.),
                ));
            }
        })
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("Player 2"),
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font: asset_server.load("fonts/pixeloid_mono.ttf"),
                    font_size: 28.,
                    ..Default::default()
                },
                Transform::from_xyz(GRID_SIZE * 0.5, GRID_SIZE * 4.5, 100.0),
            ));
        });
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn player_shooting(
    mut commands: Commands,
    mut shooting_event: EventReader<ShootingEvent>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut AseAnimation, &Player), With<Player>>,
    mut player_hit_event: EventWriter<PlayerHit>,
    mut reset_key_event: EventWriter<ResetKeysEvent>,
) {
    for event in shooting_event.read() {
        if event.player == 1 {
            if event.states.iter().filter(|d| d.is_pressed_correct).count() == 5 {
                for (mut player_animation, player) in player_query.iter_mut() {
                    if player.player_number == 1 {
                        player_animation.animation = Animation::tag("Firing")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Count(0))
                            .with_then("idle", AnimationRepeat::Loop)
                            .with_speed(1.);
                    }
                }

                commands.spawn((
                    GunShotSound,
                    AudioPlayer::new(asset_server.load("sounds/GunShot.ogg")),
                    PlaybackSettings::ONCE
                        .with_spatial(true)
                        .with_volume(Volume::Linear(1.0)),
                    Transform::from_xyz(GRID_SIZE * 5., 0., 1000.),
                ));

                player_hit_event.write(PlayerHit(2));
                reset_key_event.write(ResetKeysEvent);
            }
        }

        if event.player == 2 {
            if event.states.iter().filter(|d| d.is_pressed_correct).count() == 5 {
                for (mut player_animation, player) in player_query.iter_mut() {
                    if player.player_number == 2 {
                        player_animation.animation = Animation::tag("Firing")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Count(0))
                            .with_then("idle", AnimationRepeat::Loop)
                            .with_speed(1.);
                    }
                }

                commands.spawn((
                    GunShotSound,
                    AudioPlayer::new(asset_server.load("sounds/GunShot.ogg")),
                    PlaybackSettings::ONCE
                        .with_spatial(true)
                        .with_volume(Volume::Linear(1.0)),
                    Transform::from_xyz(-GRID_SIZE * 5., 0., 1000.),
                ));

                player_hit_event.write(PlayerHit(1));
                reset_key_event.write(ResetKeysEvent);
            }
        }
    }
}

pub fn player_hearts_status_update(
    mut player_hit_event: EventReader<PlayerHit>,
    mut player_herts_status: ResMut<PlayerHertsStatus>,
    mut player_query: Query<(&mut AseAnimation, &Player), (With<Player>, Without<PlayterHeart>)>,
    mut player_heart_query: Query<
        (&mut AseAnimation, &PlayterHeart),
        (With<PlayterHeart>, Without<Player>),
    >,
    mut check_is_game_over_event: EventWriter<CheckIsGameOverEvent>,
) {
    for event in player_hit_event.read() {
        if event.0 == 1 {
            player_herts_status.player_1_hearts =
                player_herts_status.player_1_hearts.saturating_sub(1);

            for (mut animation, heart) in player_heart_query.iter_mut() {
                if heart.0 == 1 && heart.1 == player_herts_status.player_1_hearts {
                    animation.animation = Animation::tag("Empty")
                        .with_speed(1.)
                        .with_repeat(AnimationRepeat::Loop);
                }
            }

            for (mut animation, player) in player_query.iter_mut() {
                if player.player_number == 1 {
                    animation.animation = Animation::tag("GotHit")
                        .with_speed(1.)
                        .with_repeat(AnimationRepeat::Count(2))
                        .with_then("idle", AnimationRepeat::Loop)
                        .with_speed(1.);
                }
            }

            if player_herts_status.player_1_hearts == 0 {
                check_is_game_over_event.write(CheckIsGameOverEvent);
            }
        }

        if event.0 == 2 {
            player_herts_status.player_2_hearts =
                player_herts_status.player_2_hearts.saturating_sub(1);

            for (mut animation, heart) in player_heart_query.iter_mut() {
                if heart.0 == 2 && heart.1 == player_herts_status.player_2_hearts {
                    animation.animation = Animation::tag("Empty")
                        .with_speed(1.)
                        .with_repeat(AnimationRepeat::Loop);
                }
            }

            for (mut animation, player) in player_query.iter_mut() {
                if player.player_number == 2 {
                    animation.animation = Animation::tag("GotHit")
                        .with_speed(1.)
                        .with_repeat(AnimationRepeat::Count(2))
                        .with_then("idle", AnimationRepeat::Loop)
                        .with_speed(1.);
                }
            }

            if player_herts_status.player_2_hearts == 0 {
                check_is_game_over_event.write(CheckIsGameOverEvent);
            }
        }
    }
}

pub fn change_state_to_game_over(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut check_is_game_over_event: EventReader<CheckIsGameOverEvent>,
) {
    for _ in check_is_game_over_event.read() {
        next_game_state.set(GameState::GameOver);
    }
}
