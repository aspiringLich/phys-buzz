use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::render::rounded_draw_mode;

pub fn setup_system(mut commands: Commands) {
    // spawn camera
    commands.spawn_bundle(Camera2dBundle::default());

    let radius = 100.0;
    let shape = shapes::Circle {
        center: Vec2::ZERO,
        radius,
    };
    let line = shapes::Line(Vec2::ZERO, Vec2::new(radius, 0.0));
    let mut builder = GeometryBuilder::new();
    builder = builder.add(&shape).add(&line);

    commands
        .spawn_bundle(builder.build(rounded_draw_mode(10.0, Color::WHITE), Transform::default()));
}
