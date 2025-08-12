use bevy::prelude::*;

use crate::{
    LoggedInState,
    connection::ConnectionState,
    ui::join_game::{IsHost, MatchIdInput},
};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    #[default]
    MainMenu,
    PlayNow,
    JoinGame,
    None,
}

const MAIN_MENU_BEFORE_LOGGEDIN_LIST: [&str; 2] = ["Login", "Quit"];
const MAIN_MENU_AFTER_LOGGEDIN_LIST: [&str; 3] = ["Play Now", "Join", "Quit"];

pub fn spawn_main_menu_before_logged_in(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MAIN_MENU_BEFORE_LOGGEDIN_LIST.iter().for_each(|label| {
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

pub fn spawn_main_menu_after_logged_in(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MAIN_MENU_AFTER_LOGGEDIN_LIST.iter().for_each(|label| {
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
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_connection_state: ResMut<NextState<ConnectionState>>,
    mut is_host: ResMut<IsHost>,
    mut match_id_input: ResMut<MatchIdInput>,
    mut next_logged_in_state: ResMut<NextState<LoggedInState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Login" => {
                if webbrowser::open("http://localhost:3000").is_ok() {
                    println!("Opening browser to login page...");
                }
            }
            "Play Now" => {
                is_host.0 = true;
                next_main_menu_state.set(MainMenuState::PlayNow);
                next_logged_in_state.set(LoggedInState::InGame);
            }
            "Join" => {
                is_host.0 = false;
                match_id_input.0.clear();
                next_main_menu_state.set(MainMenuState::JoinGame);
            }
            "Quit" => {
                next_connection_state.set(ConnectionState::Idle);
                std::process::exit(0)
            }
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
