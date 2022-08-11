use bevy::prelude::*;
use physics::distance;

#[derive(Clone, Copy, Debug, Default)]
pub struct Collision {
    pub force_origin: Vec2,
    pub force_vec: Vec2,
}

pub trait PhysicsObject {
    /// get the current mass of the physics object
    fn mass(&self) -> f32;
    /// get the position (center of mass) of the physics object
    fn position(&self) -> Vec2;
    /// get the current velocity of the physics object
    fn velocity(&self) -> Vec2;
    /// get the current rotation of the physics object
    fn rotation(&self) -> f32;
    /// get the current rotational velocity of an object
    fn rotational_velocity(&self) -> f32;
    /// get the moment of inertia of an object
    fn mmoi(&self) -> f32;

    /// get a mutable reference to the current mass of the physics object
    fn mass_mut(&mut self) -> &mut f32;
    /// get a mutable reference to the position (center of mass) of the physics object
    fn position_mut(&mut self) -> &mut Vec2;
    /// get a mutable reference to the current velocity of the physics object
    fn velocity_mut(&mut self) -> &mut Vec2;
    /// get a mutable reference to the current rotation of the physics object
    fn rotation_mut(&mut self) -> &mut f32;
    /// get a mutable reference to the current rotational velocity of an object
    fn rotational_velocity_mut(&mut self) -> &mut f32;
    /// get a mutable reference to the moment of inertia of an object
    fn mmoi_mut(&mut self) -> &mut f32;

    /// calculate the moment of inertia
    fn calculate_mmoi(&mut self);

    /// get the radius of a sphere encapsulating the physics object
    fn bounding_sphere_radius(&self) -> f32;

    /// detect if you have collided with another physics object
    ///     return:
    ///     Vec2 - collision origin
    ///     Vec2 - collision strength
    fn detect_collision(&self, other: &Self) -> Option<Collision>;
    /// apply a force to the physics object
    fn apply_impulse(&mut self, collision: Collision);

    /// Detect if two physics object's bounding spheres are intersecting
    fn in_bounding_sphere(&self, other: &Self) -> bool {
        distance(self.position(), other.position())
            < self.bounding_sphere_radius() + other.bounding_sphere_radius()
    }
}
