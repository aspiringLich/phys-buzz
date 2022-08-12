use bevy::prelude::*;
use physics::distance;

#[derive(Clone, Copy, Debug, Default)]
pub struct Collision {
    pub force_origin: Vec2,
    pub force_vec: Vec2,
}

pub trait PhysicsInterface {
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
}

pub trait PhysicsObject: PhysicsInterface {
    /// calculate the moment of inertia
    fn calculate_mmoi(&mut self);

    /// get the radius of a sphere encapsulating the physics object
    fn bounding_sphere_radius(&self) -> f32;

    /// detect if you have collided with another physics object
    ///     return:
    ///     Option<Collision>
    fn detect_collision(&self, other: &Self) -> Option<Collision>;

    /// apply a force to the physics object
    fn apply_impulse(&mut self, collision: Collision) {
        let force_offset = collision.force_origin - self.position();
        let force_normal = match collision.force_vec.try_normalize() {
            Some(x) => x,
            None => return,
        };
        let force_magnitude = collision.force_vec.length();

        let mass = self.mass();
        let velocity = self.velocity();
        let rotational_velocity = self.rotational_velocity();
        let mmoi = self.mmoi();

        /*
        j = force magnitude
        m = mass
        n = force normal
        I = mmoi
        r = force offset (from center of mass)

        v' = v - j / m * n
        w' = w - j * I * (r x n)
        */

        *self.velocity_mut() = velocity + (force_magnitude / mass) * force_normal;
        *self.rotational_velocity_mut() =
            rotational_velocity + (force_magnitude * mmoi * force_offset.perp_dot(force_normal))
    }

    /// Detect if two physics object's bounding spheres are intersecting
    fn in_bounding_sphere(&self, other: &Self) -> bool {
        distance(self.position(), other.position())
            < self.bounding_sphere_radius() + other.bounding_sphere_radius()
    }
}

/// macro to implement [`PhysicsInterface`] automagically for a struct
macro_rules! impl_physics_interface {
    ($($t:ty),+ $(,)?) => ($(
        impl PhysicsInterface for $t {
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
    )+)
}
