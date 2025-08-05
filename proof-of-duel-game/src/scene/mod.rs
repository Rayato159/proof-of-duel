use bevy::prelude::*;

#[derive(Component)]
pub struct InGameBackground;

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
