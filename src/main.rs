extern crate bevy;
extern crate bevy_prototype_lyon;
#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod setup;
use setup::*;

mod render;
use render::*;

mod physics;
use physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}
