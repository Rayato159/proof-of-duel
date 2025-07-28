use bevy::{audio::Volume, prelude::*};
use bevy_aseprite_ultra::prelude::*;

use crate::{
    GRID_SIZE, MAP_SIZE_X,
    shooting::{DuelRound, ShootingEvent, ShootingStatesContainer},
};

#[derive(Component)]
pub struct Player1 {
    pub wallet: String,
}

impl Player1 {
    pub fn new(wallet: String) -> Self {
        Player1 { wallet }
    }
}

#[derive(Component)]
pub struct Player2 {
    pub wallet: String,
}

impl Player2 {
    pub fn new(wallet: String) -> Self {
        Player2 { wallet }
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
pub struct Playter1Heart(pub usize);

#[derive(Component, Clone)]
pub struct Playter2Heart(pub usize);

#[derive(Event)]
pub struct PlayerHit(pub usize);

pub fn setup_player_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load("sprites/Player1.aseprite");

    commands
        .spawn((
            Player1::new("".to_string()),
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
                    Playter1Heart(i),
                    Name::new(format!("Heart_{}", i)),
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

pub fn despawn_player_1(mut commands: Commands, query: Query<Entity, With<Player1>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_player_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load("sprites/Player2.aseprite");

    commands
        .spawn((
            Player2::new("".to_string()),
            AseAnimation {
                aseprite,
                animation: Animation::tag("idle").with_speed(1.),
            },
            Sprite::default(),
        ))
        .insert(Transform::from_xyz(
            (GRID_SIZE * MAP_SIZE_X as f32) / 2. - (GRID_SIZE * 5.),
            -(GRID_SIZE * 5.),
            100.,
        ))
        .with_children(|parent| {
            for i in 0..5 {
                parent.spawn((
                    Playter2Heart(i),
                    Name::new(format!("Heart_{}", i)),
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

pub fn despawn_player_2(mut commands: Commands, query: Query<Entity, With<Player2>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn player_1_shooting(
    mut commands: Commands,
    mut shooting_event: EventReader<ShootingEvent>,
    shooting_states_container: Res<ShootingStatesContainer>,
    duel_round: Res<DuelRound>,
    asset_server: Res<AssetServer>,
    mut player_1_query: Query<&mut AseAnimation, With<Player1>>,
    mut player_hit_event: EventWriter<PlayerHit>,
) {
    for event in shooting_event.read() {
        if event.player == 1 {
            let shooting_state = shooting_states_container
                .states
                .get(duel_round.current_round - 2);

            if let Some(state) = shooting_state {
                if state.data.iter().filter(|d| d.is_pressed_correct).count() == 5 {
                    for mut player_1_animation in player_1_query.iter_mut() {
                        player_1_animation.animation = Animation::tag("Firing")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Count(1))
                            .with_then("idle", AnimationRepeat::Loop)
                            .with_speed(1.);
                    }

                    commands.spawn((
                        AudioPlayer::new(asset_server.load("sounds/GunShot.ogg")),
                        PlaybackSettings::ONCE
                            .with_spatial(true)
                            .with_volume(Volume::Linear(1.0)),
                        Transform::from_xyz(GRID_SIZE * 5., 0., 1000.),
                    ));

                    player_hit_event.write(PlayerHit(2));
                }
            }
        }
    }
}

pub fn player_2_hearts_status_update(
    mut player_hit_event: EventReader<PlayerHit>,
    mut player_herts_status: ResMut<PlayerHertsStatus>,
    mut player_2_query: Query<&mut AseAnimation, With<Player2>>,
    mut player_2_heart_query: Query<
        (&mut AseAnimation, &Playter2Heart),
        (With<Playter2Heart>, Without<Player2>),
    >,
) {
    for event in player_hit_event.read() {
        if event.0 == 2 {
            player_herts_status.player_2_hearts =
                player_herts_status.player_2_hearts.saturating_sub(1);

            for (mut animation, heart) in player_2_heart_query.iter_mut() {
                if heart.0 == player_herts_status.player_2_hearts {
                    animation.animation = Animation::tag("Empty")
                        .with_speed(1.)
                        .with_repeat(AnimationRepeat::Loop);
                }
            }

            for mut animation in player_2_query.iter_mut() {
                animation.animation = Animation::tag("GotHit")
                    .with_speed(1.)
                    .with_repeat(AnimationRepeat::Count(2))
                    .with_then("idle", AnimationRepeat::Loop)
                    .with_speed(1.);
            }
        }
    }
}
