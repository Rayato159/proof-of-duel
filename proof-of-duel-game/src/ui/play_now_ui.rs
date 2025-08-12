use arboard::Clipboard;
use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClient;
use serde::{Deserialize, Serialize};

use crate::{
    ClientChannel, ClientMessage, GameState, LoggedInState,
    connection::ConnectionState,
    player::{PlayerSelection, PlayersCounting},
    ui::{join_game::IsHost, main_menu::MainMenuState},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayersCount(pub usize);

#[derive(Component)]
pub struct PlayNowUI;

#[derive(Component)]
pub struct PlayNowText;

#[derive(Component)]
pub struct CopyMatchIdButton;

#[derive(Resource)]
pub struct GameStartTimer {
    pub timer: Timer,
    pub active: bool,
}

impl GameStartTimer {
    pub fn new(secs: f32) -> Self {
        Self {
            active: false,
            timer: Timer::from_seconds(secs, TimerMode::Once),
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.timer.reset();
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
            parent
                .spawn((
                    CopyMatchIdButton,
                    Name::new("Copy Match ID"),
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
                                Text::new("Copy Match ID"),
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
                                Text::new("Back"),
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

pub fn create_room(
    mut client: ResMut<QuinnetClient>,
    player_selection: Res<PlayerSelection>,
    is_host: Res<IsHost>,
) {
    if !is_host.0 {
        return;
    }

    let _ = client.connection_mut().send_message_on(
        ClientChannel::Lobby,
        ClientMessage::CreateMatchRequest {
            match_id: player_selection.1,
            player_wallet: "".to_string(),
        },
    );
}

pub fn update_play_now_text(
    mut text_query: Query<&mut Text, With<PlayNowText>>,
    player_counting: Res<PlayersCounting>,
    game_start_timer: Res<GameStartTimer>,
) {
    if !game_start_timer.active {
        for mut text in text_query.iter_mut() {
            *text = Text::new(format!("Waiting for players: {}/2", player_counting.0));
        }
    }
}

pub fn update_game_start_countdown(
    mut commands: Commands,
    time: Res<Time>,
    mut countdown: ResMut<GameStartTimer>,
    mut text_query: Query<&mut Text, With<PlayNowText>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    copy_match_id_button_query: Query<Entity, With<CopyMatchIdButton>>,
) {
    if !countdown.active {
        return;
    }

    for entity in copy_match_id_button_query.iter() {
        commands.entity(entity).despawn();
    }

    countdown.timer.tick(time.delta());

    for mut text in text_query.iter_mut() {
        let remaining = countdown.timer.remaining_secs().ceil();
        *text = Text::new(format!("Game starts in: {}", remaining));
    }

    if countdown.timer.finished() {
        countdown.active = false;
        next_main_menu_state.set(MainMenuState::None);
        next_game_state.set(GameState::InGame);
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
    mut connection_state: ResMut<NextState<ConnectionState>>,
    mut player_selection: ResMut<PlayerSelection>,
    mut next_logged_in_state: ResMut<NextState<LoggedInState>>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Copy Match ID" => {
                if let Ok(mut clipboard) = Clipboard::new() {
                    clipboard.set_text(player_selection.1.to_string()).ok();
                }
            }
            "Back" => {
                connection_state.set(ConnectionState::Idle);

                player_selection.reset();

                next_main_menu_state.set(MainMenuState::MainMenu);
                next_game_state.set(GameState::MainMenu);
                next_logged_in_state.set(LoggedInState::LoggedIn);
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
