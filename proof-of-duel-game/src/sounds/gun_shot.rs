use bevy::prelude::*;

#[derive(Component)]
pub struct GunShotSound;

pub fn clear_gun_shot_sound(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_shot_sound_query: Query<Entity, With<GunShotSound>>,
) {
    let mut timer = Timer::from_seconds(3.0, TimerMode::Once);
    if timer.tick(time.delta()).finished() {
        for entity in gun_shot_sound_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}
