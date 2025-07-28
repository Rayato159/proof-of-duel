use bevy::prelude::*;

use crate::player::PlayerHertsStatus;

#[derive(Component)]
pub struct InGameBackground;

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("sprites/BG.png");

    commands.spawn((
        InGameBackground,
        Sprite::from_image(background_image),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn despawn_background(mut commands: Commands, query: Query<Entity, With<InGameBackground>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_who_is_winner(
    mut commands: Commands,
    player_herts_status: Res<PlayerHertsStatus>,
    asset_server: Res<AssetServer>,
) {
    let font_bold = asset_server.load("fonts/pixeloid_mono_bold.ttf");

    if player_herts_status.player_1_hearts == 0 {
        // Player 2 wins
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
                            Text::new("Player 2 Wins!"),
                            TextColor(Color::WHITE),
                            TextLayout::new_with_justify(JustifyText::Center),
                            TextFont {
                                font: font_bold.clone(),
                                font_size: 64.,
                                ..Default::default()
                            },
                        ));
                    });
            });
    } else if player_herts_status.player_2_hearts == 0 {
        // Player 1 wins
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
                            Text::new("Player 1 Wins!"),
                            TextColor(Color::WHITE),
                            TextLayout::new_with_justify(JustifyText::Center),
                            TextFont {
                                font: font_bold.clone(),
                                font_size: 64.,
                                ..Default::default()
                            },
                        ));
                    });
            });
    }
}

pub fn despawn_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
