use bevy::prelude::*;

#[derive(Component)]
pub struct ProfileUI;

#[derive(Component)]
pub struct UserNameText;

#[derive(Component)]
pub struct BalanceText;

#[derive(Resource, Default, Clone)]
pub struct ProfileData {
    public_key: String,
    username: String,
    balance: u64,
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
                        Text::new("Username: Unknown"),
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
                        BalanceText,
                        Text::new("Coin: 0"),
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
