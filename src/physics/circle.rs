use bevy::prelude::*;
use physics::PhysicsObject;

pub struct Circle {
    radius: f32,
    mass: f32,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    rotational_velocity: f32,
    mmoi: f32,
}

impl PhysicsObject for Circle {
    fn calculate_mmoi(&mut self) {
        todo!()
    }

    fn bounding_sphere_radius(&self) -> f32 {
        todo!()
    }

    fn detect_collision(&self, other: &Self) -> Option<super::object::Collision> {
        todo!()
    }

    fn apply_impulse(&mut self, collision: super::object::Collision) {
        todo!()
    }

    /* boring funcs replace with macro later? */
    fn mass(&self) -> f32 {
        self.mass
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn velocity(&self) -> Vec2 {
        self.velocity
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn rotational_velocity(&self) -> f32 {
        self.rotational_velocity
    }

    fn mmoi(&self) -> f32 {
        self.mmoi
    }

    fn mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }

    fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    fn velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    fn rotation_mut(&mut self) -> &mut f32 {
        &mut self.rotation
    }

    fn rotational_velocity_mut(&mut self) -> &mut f32 {
        &mut self.rotational_velocity
    }

    fn mmoi_mut(&mut self) -> &mut f32 {
        &mut self.mmoi
    }
}
