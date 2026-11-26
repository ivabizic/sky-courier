use bevy::prelude::*;

mod city;
mod shared;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(city::CityPlugin)
        .run();
}