use arboard::Clipboard;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_quinnet::client::QuinnetClient;
use uuid::Uuid;

use crate::{
    ClientChannel, ClientMessage, GameState, LoggedInState, connection::ConnectionState,
    player::PlayerSelection, ui::main_menu::MainMenuState,
};

#[derive(Component)]
pub struct JoinGameUI;

#[derive(Event)]
pub struct MatchNotFoundError(pub String);

impl MatchNotFoundError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

#[derive(Component)]
pub struct MatchIdTextInput;

#[derive(Resource, Default)]
pub struct MatchIdInput(pub String);

#[derive(Resource)]
pub struct BackspaceTimer(pub Timer);

impl Default for BackspaceTimer {
    fn default() -> Self {
        BackspaceTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
pub struct IsHost(pub bool);

impl IsHost {
    pub fn reset(&mut self) {
        self.0 = false;
    }
}

pub fn spawn_join_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    match_id_input: Res<MatchIdInput>,
) {
    let font = asset_server.load("fonts/pixeloid_mono.ttf");
    let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");

    commands
        .spawn((
            JoinGameUI,
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
                        Text::new("Join Game".to_string()),
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
        .with_children(|parent_1| {
            parent_1
                .spawn({
                    Node {
                        width: Val::Percent(100.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }
                })
                .with_children(|parent_2| {
                    parent_2.spawn((
                        Text::new("Match ID".to_string()),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Left),
                        TextFont {
                            font: font.clone(),
                            font_size: 48.,
                            ..Default::default()
                        },
                        Node {
                            margin: UiRect {
                                left: Val::Px(0.),
                                right: Val::Px(32.),
                                top: Val::Px(0.),
                                bottom: Val::Px(0.),
                            },
                            ..Default::default()
                        },
                    ));
                })
                .with_children(|parent_2| {
                    parent_2
                        .spawn((
                            Node {
                                width: Val::Px(800.),
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
                        .with_children(|parent_3| {
                            parent_3
                                .spawn((Node {
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    position_type: PositionType::Relative,
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    padding: UiRect::all(Val::Px(8.)),
                                    ..Default::default()
                                },))
                                .with_children(|parent_4| {
                                    parent_4.spawn((
                                        MatchIdTextInput,
                                        Text::new(match_id_input.0.clone()),
                                        TextColor(Color::WHITE),
                                        TextLayout::new_with_justify(JustifyText::Left),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 32.,
                                            ..Default::default()
                                        },
                                    ));
                                });
                        });
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Name::new("Join"),
                    Button,
                    Node {
                        width: Val::Px(360.),
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
                                Text::new("Join".to_string()),
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

            parent
                .spawn((
                    Name::new("Back"),
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
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(48.),
                            bottom: Val::Px(0.),
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
                                Text::new("Back".to_string()),
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
        });
}

pub fn update_match_id_input(
    mut char_evr: EventReader<KeyboardInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut buffer: ResMut<MatchIdInput>,
    mut text_query: Query<&mut Text, With<MatchIdTextInput>>,
    mut backspace_timer: ResMut<BackspaceTimer>,
    time: Res<Time>,
) {
    // Handle backspace
    if keyboard_input.pressed(KeyCode::Backspace) {
        backspace_timer.0.tick(time.delta());

        if backspace_timer.0.just_finished() {
            buffer.0.pop();
        }
    } else {
        backspace_timer.0.reset();
    }

    // Handle paste (Ctrl + V)
    if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_pressed(KeyCode::KeyV) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Ok(clip_text) = clipboard.get_text() {
                for c in clip_text.chars() {
                    if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                        buffer.0.push(c);
                    }
                }
            }
        }
    }

    // Handle typed characters
    for ev in char_evr.read() {
        if let Some(t) = ev.text.clone() {
            t.chars().for_each(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    buffer.0.push(c);
                }
            });
        }
    }

    // Update the UI
    for mut text in text_query.iter_mut() {
        if buffer.0.len() > 36 {
            buffer.0.truncate(36); // Limit to 36 characters
        }

        *text = Text::new(buffer.0.clone());
    }
}

pub fn join_game_ui_interaction(
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

pub fn join_game_button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut connection_state: ResMut<NextState<ConnectionState>>,
    mut player_selection: ResMut<PlayerSelection>,
    mut buffer: ResMut<MatchIdInput>,
    mut client: ResMut<QuinnetClient>,
    mut next_logged_in_state: ResMut<NextState<LoggedInState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(match_id) = Uuid::parse_str(&buffer.0) else {
            continue;
        };

        match name.as_str() {
            "Join" => {
                let _ = client.connection_mut().send_message_on(
                    ClientChannel::Lobby,
                    ClientMessage::JoinMatchRequest {
                        match_id,
                        player_wallet: "".to_string(),
                    },
                );

                player_selection.1 = match_id;

                next_main_menu_state.set(MainMenuState::PlayNow);
                next_logged_in_state.set(LoggedInState::InGame);
            }
            "Back" => {
                connection_state.set(ConnectionState::Idle);

                player_selection.reset();
                buffer.0.clear();

                next_main_menu_state.set(MainMenuState::MainMenu);
                next_game_state.set(GameState::MainMenu);
                next_logged_in_state.set(LoggedInState::LoggedIn);
            }
            _ => return,
        }
    }
}

pub fn match_not_found_error(
    mut commands: Commands,
    join_game_ui_query: Query<Entity, With<JoinGameUI>>,
    asset_server: Res<AssetServer>,
    mut match_not_found_error_event: EventReader<MatchNotFoundError>,
) {
    for event in match_not_found_error_event.read() {
        let error_message = event.0.to_owned();

        let font = asset_server.load("fonts/pixeloid_mono.ttf");
        let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");

        for entity in join_game_ui_query.iter() {
            commands.entity(entity).despawn();
        }

        commands
            .spawn((
                JoinGameUI,
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
                            Text::new(error_message),
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
                parent
                    .spawn((
                        Name::new("Back"),
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
                            margin: UiRect {
                                left: Val::Px(0.),
                                right: Val::Px(0.),
                                top: Val::Px(48.),
                                bottom: Val::Px(0.),
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
                                    Text::new("Back".to_string()),
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
            });
    }
}

pub fn reset_is_host(mut is_host: ResMut<IsHost>) {
    is_host.reset();
}

pub fn despawn_join_game_ui(
    mut commands: Commands,
    join_game_ui_query: Query<Entity, With<JoinGameUI>>,
) {
    for entity in join_game_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}
