use bevy::prelude::*;

use crate::stats::StatsData;

#[derive(Component)]
pub struct ProfileUI;

#[derive(Component)]
pub struct UserNameText;

#[derive(Component)]
pub struct WinText;

#[derive(Component)]
pub struct LossText;

#[derive(Resource, Default, Clone)]
pub struct ProfileData {
    pub logged_in: bool,
    pub public_key: String,
    pub username: String,
}

pub fn spawn_profile_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/pixeloid_mono.ttf");

    commands
        .spawn((
            ProfileUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(24.),
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        padding: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(8.),
                            bottom: Val::Px(8.),
                        },
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
                .with_children(|parent| {
                    parent.spawn((
                        UserNameText,
                        Text::new("Username: "),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Left),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        padding: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(8.),
                            bottom: Val::Px(8.),
                        },
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
                .with_children(|parent| {
                    parent.spawn((
                        WinText,
                        Text::new("Win: 0"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Left),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.,
                            ..Default::default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        padding: UiRect {
                            left: Val::Px(8.),
                            right: Val::Px(8.),
                            top: Val::Px(8.),
                            bottom: Val::Px(8.),
                        },
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
                .with_children(|parent| {
                    parent.spawn((
                        LossText,
                        Text::new("Loss: 0"),
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Left),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.,
                            ..Default::default()
                        },
                    ));
                });
        });
}

pub fn update_username(
    profile_data: Res<ProfileData>,
    mut query: Query<&mut Text, With<UserNameText>>,
) {
    for mut text in query.iter_mut() {
        *text = format!("Username: {}", profile_data.username.to_owned()).into();
    }
}

pub fn update_win(stats_data: Res<StatsData>, mut query: Query<&mut Text, With<WinText>>) {
    for mut text in query.iter_mut() {
        *text = format!("Win: {}", stats_data.win.to_owned()).into();
    }
}

pub fn update_loss(stats_data: Res<StatsData>, mut query: Query<&mut Text, With<LossText>>) {
    for mut text in query.iter_mut() {
        *text = format!("Loss: {}", stats_data.loss.to_owned()).into();
    }
}
