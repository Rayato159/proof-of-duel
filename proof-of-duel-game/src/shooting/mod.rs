use crate::{
    ClientChannel, ClientMessage, GRID_SIZE,
    player::{PlayerSelection, ShootingLock},
    shooting::keycode::ALL_KEYS,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_quinnet::client::QuinnetClient;
use rand::prelude::*;

pub mod keycode;

pub const KEY_POOL: [KeyCode; 4] = [KeyCode::KeyQ, KeyCode::KeyW, KeyCode::KeyE, KeyCode::KeyR];

#[derive(Resource, Debug, Clone)]
pub struct ShootingStates {
    pub data: [ShootingData; 5],
    pub current_key_index: usize,
    pub wrong_count: usize,
}

impl Default for ShootingStates {
    fn default() -> Self {
        let data = [(); 5].map(|_| ShootingData {
            key: KEY_POOL[rand::rng().random_range(0..KEY_POOL.len())],
            is_pressed_correct: false,
        });

        ShootingStates {
            data,
            current_key_index: 0,
            wrong_count: 0,
        }
    }
}

impl ShootingStates {
    pub fn is_last_key(&self) -> bool {
        self.current_key_index == 5
    }

    pub fn reset_current_key_index(&mut self) {
        self.current_key_index = 0;
    }

    pub fn next_key(&mut self) {
        self.current_key_index = (self.current_key_index + 1).clamp(0, 5);
    }

    pub fn wrong_key_increment(&mut self) {
        self.wrong_count += 1;
    }

    pub fn randomize_keys(&mut self) {
        for data in self.data.iter_mut() {
            data.key = KEY_POOL[rand::rng().random_range(0..KEY_POOL.len())];
            data.is_pressed_correct = false;
        }
    }

    pub fn reset(&mut self) {
        self.data = [(); 5].map(|_| ShootingData {
            key: KEY_POOL[rand::rng().random_range(0..KEY_POOL.len())],
            is_pressed_correct: false,
        });
        self.current_key_index = 0;
        self.wrong_count = 0;
    }
}

#[derive(Component, Debug, Clone)]
pub struct ShootingKeyIndex(pub usize);

#[derive(Debug, Clone)]
pub struct ShootingData {
    pub key: KeyCode,
    pub is_pressed_correct: bool,
}

#[derive(Component)]
pub struct ShootingKey;

#[derive(Event)]
pub struct ResetKeysEvent;

#[derive(Event, Default)]
pub struct CheckShootingKeyEvent(pub usize);

#[derive(Event)]
pub struct ShootingEvent {
    pub player: usize,
}

pub fn spawn_shooting_keys(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    shooting_states: Res<ShootingStates>,
) {
    for (i, data) in shooting_states.data.iter().enumerate() {
        let pos = Vec3::new(
            (-GRID_SIZE * 5.) + (i as f32 * (7. + 64.)),
            -(GRID_SIZE * 1.),
            100.,
        );

        commands
            .spawn((
                ShootingKey,
                ShootingKeyIndex(i),
                AseAnimation {
                    aseprite: asset_server.load("sprites/Key.aseprite"),
                    animation: Animation::tag("idle").with_speed(1.),
                },
                Transform::from_translation(pos),
                Sprite::default(),
            ))
            .with_children(|parent| {
                let key_label = match data.key {
                    KeyCode::KeyQ => "Q",
                    KeyCode::KeyW => "W",
                    KeyCode::KeyE => "E",
                    KeyCode::KeyR => "R",
                    _ => "",
                };

                parent.spawn((
                    Text2d::new(key_label),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont {
                        font: asset_server.load("fonts/pixeloid_mono.ttf"),
                        font_size: 36.,
                        ..Default::default()
                    },
                    Transform::from_xyz(0., 0., 200.),
                ));
            });
    }
}

pub fn despawn_shooting_keys(
    mut commands: Commands,
    shooting_keys_query: Query<Entity, With<ShootingKey>>,
) {
    for entity in shooting_keys_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn shooting_key_input(
    mut shooting_key_query: Query<(&mut AseAnimation, &ShootingKeyIndex), With<ShootingKey>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shooting_states: ResMut<ShootingStates>,
    mut reset_key_event: EventWriter<ResetKeysEvent>,
    player_slecrion: Res<PlayerSelection>,
    mut client: ResMut<QuinnetClient>,
    mut shooting_lock: ResMut<ShootingLock>,
) {
    let current_key_index = shooting_states.current_key_index;

    if let Some(data) = shooting_states.data.get_mut(current_key_index) {
        if keyboard_input.any_just_pressed(ALL_KEYS) {
            if keyboard_input.just_pressed(data.key) {
                for (mut shooting_key_anim, key_index) in shooting_key_query.iter_mut() {
                    if key_index.0 == current_key_index {
                        shooting_key_anim.animation = Animation::tag("Correct")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Loop);
                    }
                }

                data.is_pressed_correct = true;
                shooting_states.next_key();
            } else {
                for (mut shooting_key_anim, key_index) in shooting_key_query.iter_mut() {
                    if key_index.0 == current_key_index {
                        shooting_key_anim.animation = Animation::tag("InCorrect")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Loop);
                    }
                }

                shooting_states.wrong_key_increment();
                shooting_states.reset_current_key_index();
                reset_key_event.write(ResetKeysEvent);
            }

            if shooting_states.is_last_key() {
                shooting_lock.lock();

                if shooting_lock.is_locked() {
                    let _ = client.connection_mut().send_message_on(
                        ClientChannel::Shooting,
                        &ClientMessage::ShootingCommand {
                            match_id: player_slecrion.1,
                            player_number: player_slecrion.0,
                        },
                    );
                }

                shooting_lock.unlock();

                shooting_states.reset_current_key_index();
            }
        }
    }
}

pub fn spawn_new_shooting_keys(
    mut reset_key_event: EventReader<ResetKeysEvent>,
    mut commands: Commands,
    shooting_keys_query: Query<Entity, With<ShootingKey>>,
    asset_server: Res<AssetServer>,
    mut shooting_states: ResMut<ShootingStates>,
) {
    for _ in reset_key_event.read() {
        for entity in shooting_keys_query.iter() {
            commands.entity(entity).despawn();
        }

        shooting_states.randomize_keys();

        for (i, data) in shooting_states.data.iter().enumerate() {
            let pos = Vec3::new(
                (-GRID_SIZE * 5.) + (i as f32 * (7. + 64.)),
                -(GRID_SIZE * 1.),
                100.,
            );

            commands
                .spawn((
                    ShootingKey,
                    ShootingKeyIndex(i),
                    AseAnimation {
                        aseprite: asset_server.load("sprites/Key.aseprite"),
                        animation: Animation::tag("idle").with_speed(1.),
                    },
                    Transform::from_translation(pos),
                    Sprite::default(),
                ))
                .with_children(|parent| {
                    let key_label = match data.key {
                        KeyCode::KeyQ => "Q",
                        KeyCode::KeyW => "W",
                        KeyCode::KeyE => "E",
                        KeyCode::KeyR => "R",
                        _ => "",
                    };

                    parent.spawn((
                        Text2d::new(key_label),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: asset_server.load("fonts/pixeloid_mono.ttf"),
                            font_size: 36.,
                            ..Default::default()
                        },
                        Transform::from_xyz(0., 0., 200.),
                    ));
                });
        }
    }
}
