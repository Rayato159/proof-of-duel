use bevy::{audio::Volume, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use uuid::Uuid;

use crate::{
    GRID_SIZE, MAP_SIZE_X,
    shooting::{ResetKeysEvent, ShootingEvent},
    sounds::gun_shot::GunShotSound,
};

#[derive(Resource, Debug)]
pub struct PlayerSelection(pub usize, pub Uuid);

impl Default for PlayerSelection {
    fn default() -> Self {
        Self(0, Uuid::new_v4())
    }
}

impl PlayerSelection {
    pub fn reset(&mut self) {
        self.0 = 0;
        self.1 = Uuid::new_v4();
    }
}

#[derive(Resource, Default)]
pub struct PlayersCounting(pub usize);

impl PlayersCounting {
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Debug, Clone, Component)]
pub struct Player {
    pub player_number: usize,
    pub wallet: String,
}

impl Player {
    pub fn new(player_number: usize, wallet: String) -> Self {
        Player {
            player_number,
            wallet,
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerHertsStatus {
    pub player_1_hearts: usize,
    pub player_2_hearts: usize,
}

impl PlayerHertsStatus {
    pub fn reset(&mut self) {
        self.player_1_hearts = 5;
        self.player_2_hearts = 5;
    }
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

#[derive(Resource, Default)]
pub struct ShootingLock(pub bool);

impl ShootingLock {
    pub fn lock(&mut self) {
        self.0 = true;
    }

    pub fn unlock(&mut self) {
        self.0 = false;
    }

    pub fn is_locked(&self) -> bool {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = false;
    }
}

pub fn setup_player_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_selection: Res<PlayerSelection>,
) {
    let aseprite = asset_server.load("sprites/Player1.aseprite");

    let player_text = if player_selection.0 == 1 {
        "You"
    } else {
        "Player 1"
    };

    let player_text_color = if player_selection.0 == 1 {
        Color::srgba(255. / 255., 222. / 255., 99. / 255., 1.0)
    } else {
        Color::WHITE
    };

    commands
        .spawn((
            Player::new(1, "".to_string()),
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
            for i in 1..=5 {
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
                Text2d::new(player_text),
                TextColor(player_text_color),
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

    let player_text = if player_selection.0 == 2 {
        "You"
    } else {
        "Player 2"
    };

    let player_text_color = if player_selection.0 == 2 {
        Color::srgba(255. / 255., 222. / 255., 99. / 255., 1.0)
    } else {
        Color::WHITE
    };

    commands
        .spawn((
            Player::new(2, "".to_string()),
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
            for i in 1..=5 {
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
                Text2d::new(player_text),
                TextColor(player_text_color),
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
    mut reset_key_event: EventWriter<ResetKeysEvent>,
    player_selection: Res<PlayerSelection>,
) {
    for event in shooting_event.read() {
        for (mut player_animation, player) in player_query.iter_mut() {
            if player.player_number == event.player {
                player_animation.animation = Animation::tag("Firing")
                    .with_speed(1.)
                    .with_repeat(AnimationRepeat::Count(0))
                    .with_then("idle", AnimationRepeat::Loop)
                    .with_speed(1.);
            }
        }

        let sound_pos = if event.player == 1 {
            Vec3::new(GRID_SIZE * 5., 0., 1000.)
        } else {
            Vec3::new(-GRID_SIZE * 5., 0., 1000.)
        };

        commands.spawn((
            GunShotSound,
            AudioPlayer::new(asset_server.load("sounds/GunShot.ogg")),
            PlaybackSettings::ONCE
                .with_spatial(true)
                .with_volume(Volume::Linear(1.0)),
            Transform::from_translation(sound_pos),
        ));

        if event.player == player_selection.0 {
            reset_key_event.write(ResetKeysEvent);
        }
    }
}

pub fn who_was_hit(
    mut player_query: Query<(&mut AseAnimation, &Player), (With<Player>, Without<PlayterHeart>)>,
    mut player_hit: EventReader<PlayerHit>,
) {
    for event in player_hit.read() {
        let target_player_number = event.0;

        // Animate player character being hit
        for (mut anim, player) in player_query.iter_mut() {
            if player.player_number == target_player_number {
                anim.animation = Animation::tag("GotHit")
                    .with_speed(1.)
                    .with_repeat(AnimationRepeat::Count(2))
                    .with_then("idle", AnimationRepeat::Loop);
            }
        }
    }
}

pub fn update_heart_status(
    mut player_heart_query: Query<(&mut AseAnimation, &PlayterHeart), With<PlayterHeart>>,
    player_hearts_status: Res<PlayerHertsStatus>,
) {
    // Animate hearts UI
    for (mut anim, heart) in player_heart_query.iter_mut() {
        if heart.0 == 1 && heart.1 > player_hearts_status.player_1_hearts {
            anim.animation = Animation::tag("Empty")
                .with_speed(1.)
                .with_repeat(AnimationRepeat::Loop);
        }

        if heart.0 == 2 && heart.1 > player_hearts_status.player_2_hearts {
            anim.animation = Animation::tag("Empty")
                .with_speed(1.)
                .with_repeat(AnimationRepeat::Loop);
        }
    }
}
