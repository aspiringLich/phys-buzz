use bevy::prelude::*;
use physics::{Collision, PhysicsInterface, PhysicsObject};

#[derive(Component)]
pub struct Circle {
    radius: f32,
    mass: f32,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    rotational_velocity: f32,
    mmoi: f32,
}

impl_physics_interface!(Circle);

impl PhysicsObject for Circle {
    fn calculate_mmoi(&mut self) {
        self.mmoi = 0.5 * self.mass() * self.radius.powi(2);
    }

    fn bounding_sphere_radius(&self) -> f32 {
        self.radius
    }

    fn detect_collision(&self, other: &Self) -> Option<Collision> {
        todo!()
    }
}
