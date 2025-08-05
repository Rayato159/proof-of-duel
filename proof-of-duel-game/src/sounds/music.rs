use bevy::{audio::Volume, prelude::*};

use crate::MUSIC_VOLUME;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn play_bg_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(asset_server.load("musics/bro_turned_into_prog_mode.ogg")),
        PlaybackSettings::ONCE.with_volume(Volume::Linear(MUSIC_VOLUME)),
    ));
}

pub fn stop_playing_bg_music(
    mut commands: Commands,
    background_music_query: Query<Entity, With<BackgroundMusic>>,
) {
    for entity in background_music_query.iter() {
        commands.entity(entity).despawn();
    }
}
