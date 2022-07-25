use bevy::prelude::*;

pub fn setup_system(mut commands: Commands) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
