use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::components::Sand;
use crate::resources::SandGrid;

pub fn spawn_sand_on_click(
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    grid: Res<SandGrid>,
) {
    if btn.pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();
        
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let mut rng = rand::thread_rng();
            
            // Spawn multiple grains of sand at once
            for _ in 0..3 {
                let offset_x = rng.gen_range(-10.0..10.0);
                let offset_y = rng.gen_range(-10.0..10.0);
                
                let spawn_pos = world_position + Vec2::new(offset_x, offset_y);
                
                // Convert world position to grid coordinates
                if let Some((grid_x, grid_y)) = grid.world_to_grid(spawn_pos) {
                    let pos = grid.grid_to_world(grid_x, grid_y);
                    
                    commands.spawn((
                        Sand::default(),
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.9, 0.7, 0.3), // Sandy color
                                custom_size: Some(Vec2::new(grid.cell_size, grid.cell_size)),
                                ..default()
                            },
                            transform: Transform::from_translation(pos.extend(0.0)),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}