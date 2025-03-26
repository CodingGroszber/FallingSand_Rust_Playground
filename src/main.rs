use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::SandGrid;
use systems::{input, physics, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling Sand".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SandGrid::new(160, 120)) // Grid size
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (
            input::spawn_sand_on_click,
            physics::update_sand,
        ))
        .run();
}