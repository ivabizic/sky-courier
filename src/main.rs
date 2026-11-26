use bevy::prelude::*;

mod camera;
mod city;
mod player;
mod shared;

use camera::CameraPlugin;
use city::CityPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, CityPlugin))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(1.0, 0.94, 0.82),
            brightness: 250.0,
            ..default()
        })
        .run();
}
