use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct GameOverUICamera;

pub fn game_camera_setup(mut commands: Commands) {
    commands.spawn((GameCamera, Camera2d, Transform::from_xyz(0.0, 0.0, 1000.)));
}

pub fn despawn_game_camera(mut commands: Commands, query: Query<Entity, With<GameCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn game_over_camera_setup(mut commands: Commands) {
    commands.spawn((
        GameOverUICamera,
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.),
    ));
}

pub fn despawn_game_over_camera(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUICamera>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
