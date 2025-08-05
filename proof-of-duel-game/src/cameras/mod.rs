use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component)]
pub struct PlayNowUICamera;

#[derive(Component)]
pub struct JoinGameUICamera;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct GameOverUICamera;

pub fn main_menu_camera_setup(mut commands: Commands) {
    commands.spawn((
        MainMenuCamera,
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.),
    ));
}

pub fn despawn_main_menu_camera(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuCamera>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn play_now_ui_camera_setup(mut commands: Commands) {
    commands.spawn((
        PlayNowUICamera,
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.),
    ));
}

pub fn despawn_play_now_ui_camera(
    mut commands: Commands,
    query: Query<Entity, With<PlayNowUICamera>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn join_game_ui_camera_setup(mut commands: Commands) {
    commands.spawn((
        JoinGameUICamera,
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.),
    ));
}

pub fn despawn_join_game_ui_camera(
    mut commands: Commands,
    query: Query<Entity, With<JoinGameUICamera>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

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
