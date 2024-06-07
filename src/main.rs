use bevy::prelude::*;

mod bevy_0_13_0_tutorial;
// use bevy_0_13_0_tutorial as bevyTut;

mod Scene3d;

fn main() {
    // uncomment for the bevy tutorial for 0.13.0
    // App::new()
    // .add_plugins((DefaultPlugins, bevyTut::HelloPlugin))
    // .run();

    // 3d scene
    App::new()
    .add_plugins((DefaultPlugins, Scene3d::Scene3d_Plugin))
    .run();
}
