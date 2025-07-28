use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn game_camera_setup(mut commands: Commands) {
    commands.spawn((GameCamera, Camera2d, Transform::from_xyz(0.0, 0.0, 1000.)));
}

pub fn despawn_game_camera(mut commands: Commands, query: Query<Entity, With<GameCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
