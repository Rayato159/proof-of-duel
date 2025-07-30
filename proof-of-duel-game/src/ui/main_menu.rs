use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{GameState, player::PlayersCounting};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PlayNowUI;

#[derive(Component)]
pub struct PlayNowText;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    #[default]
    None,
    PlayNow,
    JoinGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersCount(pub usize);

#[derive(Resource)]
pub struct GameStartTimer {
    pub is_running: bool,
    pub timer: Timer,
}

impl GameStartTimer {
    pub fn new(secs: f32) -> Self {
        Self {
            is_running: false,
            timer: Timer::from_seconds(secs, TimerMode::Once),
        }
    }
}

const MAIN_MENU_LIST: [&str; 3] = ["Hosting Lobby", "Join Game", "Quit"];
const WAITING_LIST: [&str; 1] = ["Back"];

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((
            MainMenuUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn({
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Proof of Duel"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 94.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            MAIN_MENU_LIST.iter().for_each(|label| {
                parent
                    .spawn((
                        Name::new(label.to_string()),
                        Button,
                        Node {
                            width: Val::Px(502.),
                            height: Val::Px(88.),
                            position_type: PositionType::Relative,
                            border: UiRect {
                                left: Val::Px(2.),
                                right: Val::Px(2.),
                                top: Val::Px(2.),
                                bottom: Val::Px(2.),
                            },
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        BackgroundColor(Color::WHITE.with_alpha(0.0)),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(label.to_string()),
                                    TextColor(Color::WHITE),
                                    TextLayout::new_with_justify(JustifyText::Center),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 48.,
                                        ..Default::default()
                                    },
                                ));
                            });
                    });
            })
        });
}

pub fn main_menu_button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Hosting Lobby" => {
                next_main_menu_state.set(MainMenuState::PlayNow);
            }
            "Join Game" => {
                next_game_state.set(GameState::InGame);
            }
            "Quit" => std::process::exit(0),
            _ => return,
        }
    }
}

pub fn main_menu_ui_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.15));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.07));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::NONE);
            }
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_ui_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in main_menu_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_play_now_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((
            PlayNowUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn({
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }
                })
                .with_children(|parent| {
                    parent.spawn((
                        PlayNowText,
                        Text::new("Waiting for players..."),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 64.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            WAITING_LIST.iter().for_each(|label| {
                parent
                    .spawn((
                        Name::new(label.to_string()),
                        Button,
                        Node {
                            width: Val::Px(502.),
                            height: Val::Px(88.),
                            position_type: PositionType::Relative,
                            border: UiRect {
                                left: Val::Px(2.),
                                right: Val::Px(2.),
                                top: Val::Px(2.),
                                bottom: Val::Px(2.),
                            },
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        BackgroundColor(Color::WHITE.with_alpha(0.0)),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(label.to_string()),
                                    TextColor(Color::WHITE),
                                    TextLayout::new_with_justify(JustifyText::Center),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 48.,
                                        ..Default::default()
                                    },
                                ));
                            });
                    });
            })
        });
}

pub fn update_play_now_text(
    mut text_query: Query<&mut Text, With<PlayNowText>>,
    player_counting: Res<PlayersCounting>,
) {
    for mut text in text_query.iter_mut() {
        *text = Text::new(format!("Waiting for players: {}/2", player_counting.0));
    }
}

pub fn update_game_start_text(
    time: Res<Time>,
    mut countdown: ResMut<GameStartTimer>,
    mut text_query: Query<&mut Text, With<PlayNowText>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if countdown.is_running {
        countdown.timer.tick(time.delta());

        for mut text in text_query.iter_mut() {
            let remaining = countdown.timer.remaining_secs().ceil();
            *text = Text::new(format!("Game starts in: {}", remaining));
        }

        if countdown.timer.finished() {
            next_main_menu_state.set(MainMenuState::None);
            next_game_state.set(GameState::InGame);
        }
    }
}

pub fn play_now_ui_interaction(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.15));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.07));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::NONE);
            }
        }
    }
}

pub fn play_now_button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Back" => {
                next_main_menu_state.set(MainMenuState::None);
                next_game_state.set(GameState::MainMenu);
            }
            _ => return,
        }
    }
}

pub fn despawn_play_now_ui(
    mut commands: Commands,
    play_now_ui_query: Query<Entity, With<PlayNowUI>>,
) {
    for entity in play_now_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
