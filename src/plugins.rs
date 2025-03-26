// plugins.rs
use bevy::prelude::*;
use crate::{resources::Grid, systems::{movement_system, position_sync_system, input_system}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new())
            .add_startup_system(setup_camera)
            .add_system(input_system)
            .add_system(movement_system)
            .add_system(position_sync_system.after(movement_system));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            left: 0.0,
            right: 800.0 / crate::CELL_SIZE,
            bottom: 0.0,
            top: 600.0 / crate::CELL_SIZE,
            ..default()
        },
        transform: Transform::from_xyz(
            (800.0 / crate::CELL_SIZE / 2.0) as f32,
            (600.0 / crate::CELL_SIZE / 2.0) as f32,
            0.0,
        ),
        ..default()
    });
}