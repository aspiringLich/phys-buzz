use bevy::prelude::*;

pub fn distance(a: Vec2, b: Vec2) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    return (x * x + y * y).sqrt();
}
