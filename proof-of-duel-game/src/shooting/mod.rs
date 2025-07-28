use crate::{GRID_SIZE, shooting::keycode::ALL_KEYS};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use rand::prelude::*;

pub mod keycode;

pub const KEY_POOL: [KeyCode; 4] = [KeyCode::KeyQ, KeyCode::KeyW, KeyCode::KeyE, KeyCode::KeyR];

#[derive(Debug, Clone)]
pub struct ShootingStates {
    pub data: [ShootingData; 5],
    pub current_key_index: usize,
    pub wrong_count: usize,
}

impl ShootingStates {
    pub fn is_last_key(&self) -> bool {
        self.current_key_index > 4
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
}

#[derive(Component, Debug, Clone)]
pub struct ShootingKeyIndex(pub usize);

#[derive(Debug, Clone)]
pub struct ShootingData {
    pub key: KeyCode,
    pub is_pressed_correct: bool,
}

#[derive(Resource)]
pub struct ShootingStatesContainer {
    pub states: [ShootingStates; 5],
}

impl Default for ShootingStatesContainer {
    fn default() -> Self {
        let states = [(); 5].map(|_| {
            let data = [(); 5].map(|_| ShootingData {
                key: KEY_POOL[rand::rng().random_range(0..KEY_POOL.len())],
                is_pressed_correct: false,
            });
            ShootingStates {
                data,
                current_key_index: 0,
                wrong_count: 0,
            }
        });

        Self { states }
    }
}

#[derive(Resource)]
pub struct DuelRound {
    pub current_round: usize,
}

impl Default for DuelRound {
    fn default() -> Self {
        Self { current_round: 1 }
    }
}

impl DuelRound {
    pub fn next_round(&mut self) {
        self.current_round = (self.current_round + 1).clamp(1, 6);
    }

    pub fn is_last_round(&self) -> bool {
        self.current_round > 5
    }
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
    shooting_states_container: Res<ShootingStatesContainer>,
    duel_round: Res<DuelRound>,
) {
    let current_state = shooting_states_container
        .states
        .get(duel_round.current_round - 1);

    if current_state.is_none() {
        return;
    }

    for (i, data) in current_state.unwrap().data.iter().enumerate() {
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
    mut shooting_states_container: ResMut<ShootingStatesContainer>,
    mut duel_round: ResMut<DuelRound>,
    mut reset_key_event: EventWriter<ResetKeysEvent>,
    mut shooting_event: EventWriter<ShootingEvent>,
) {
    if duel_round.is_last_round() {
        return;
    }

    let current_round_index = duel_round.current_round - 1;

    let Some(state) = shooting_states_container
        .states
        .get_mut(current_round_index)
    else {
        return;
    };

    let current_key_index = state.current_key_index;

    if let Some(data) = state.data.get_mut(current_key_index) {
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
                state.next_key();
            } else {
                for (mut shooting_key_anim, key_index) in shooting_key_query.iter_mut() {
                    if key_index.0 == current_key_index {
                        shooting_key_anim.animation = Animation::tag("InCorrect")
                            .with_speed(1.)
                            .with_repeat(AnimationRepeat::Loop);
                    }
                }

                state.wrong_key_increment();
                state.reset_current_key_index();
                reset_key_event.write(ResetKeysEvent);
            }

            if state.is_last_key() {
                duel_round.next_round();

                // Trigger the shooting event for the player
                // Assuming player 1 is the shooter
                shooting_event.write(ShootingEvent { player: 1 });

                state.reset_current_key_index();
                reset_key_event.write(ResetKeysEvent);
            }
        }
    }
}

pub fn spawn_new_shooting_keys(
    mut reset_key_event: EventReader<ResetKeysEvent>,
    mut commands: Commands,
    shooting_keys_query: Query<Entity, With<ShootingKey>>,
    asset_server: Res<AssetServer>,
    shooting_states_container: Res<ShootingStatesContainer>,
    duel_round: Res<DuelRound>,
) {
    for _ in reset_key_event.read() {
        for entity in shooting_keys_query.iter() {
            commands.entity(entity).despawn();
        }

        let current_state = shooting_states_container
            .states
            .get(duel_round.current_round - 1);

        if current_state.is_none() {
            return;
        }

        for (i, data) in current_state.unwrap().data.iter().enumerate() {
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
