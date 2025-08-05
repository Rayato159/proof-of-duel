use bevy::prelude::*;

use crate::{
    GameState,
    connection::ConnectionState,
    player::{PlayerHertsStatus, PlayerSelection, PlayersCounting, ShootingLock},
    shooting::ShootingStates,
    ui::{main_menu::MainMenuState, play_now_ui::GameStartTimer},
};

#[derive(Component)]
pub struct GameOverUI;

#[derive(Resource, Debug, Clone, Default)]
pub struct WhoIsWinner {
    pub player_number: usize,
}

impl WhoIsWinner {
    pub fn reset(&mut self) {
        self.player_number = 0;
    }
}

pub fn spawn_game_over_ui(
    mut commands: Commands,
    who_is_winner: Res<WhoIsWinner>,
    asset_server: Res<AssetServer>,
    player_selection: Res<PlayerSelection>,
) {
    let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");
    let whos_winner = if who_is_winner.player_number == 1 && player_selection.0 == 1 {
        "You Win!"
    } else if who_is_winner.player_number == 2 && player_selection.0 == 2 {
        "You Win!"
    } else if who_is_winner.player_number == 1 && player_selection.0 == 2 {
        "You Lose!"
    } else if who_is_winner.player_number == 2 && player_selection.0 == 1 {
        "You Lose!"
    } else {
        "It's a Draw!"
    };

    commands
        .spawn((
            GameOverUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
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
                        Text::new(whos_winner),
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
                .spawn((
                    Button,
                    Name::new("Back to Main Menu".to_string()),
                    Node {
                        width: Val::Px(502.),
                        height: Val::Px(88.),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
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
                ))
                .with_children(|parent_2| {
                    parent_2.spawn((
                        Text::new("Back to Main Menu"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 36.,
                            ..Default::default()
                        },
                    ));
                });
        });
}

pub fn game_over_ui_interaction(
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

pub fn game_over_button_pressed_handler(
    button_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut connection_state: ResMut<NextState<ConnectionState>>,
    mut player_selection: ResMut<PlayerSelection>,
    mut player_hearts_status: ResMut<PlayerHertsStatus>,
    mut players_counting: ResMut<PlayersCounting>,
    mut game_start_timer: ResMut<GameStartTimer>,
    mut shooting_states: ResMut<ShootingStates>,
    mut who_is_winner: ResMut<WhoIsWinner>,
    mut shooting_lock: ResMut<ShootingLock>,
) {
    for (interaction, name) in button_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match name.as_str() {
            "Back to Main Menu" => {
                connection_state.set(ConnectionState::Idle);

                player_selection.reset();
                player_hearts_status.reset();
                players_counting.reset();
                game_start_timer.reset();
                shooting_states.reset();
                who_is_winner.reset();
                shooting_lock.reset();

                main_menu_state.set(MainMenuState::MainMenu);
                next_game_state.set(GameState::MainMenu);
            }
            _ => return,
        }
    }
}

pub fn despawn_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
