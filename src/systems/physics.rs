use bevy::prelude::*;
use rand::Rng;

use crate::components::Sand;
use crate::resources::SandGrid;

pub fn update_sand(
    mut commands: Commands,
    mut sand_query: Query<(Entity, &mut Transform, &mut Sand)>,
    mut grid: ResMut<SandGrid>,
) {
    let mut rng = rand::thread_rng();
    
    // First clear the grid
    for cell in grid.cells.iter_mut() {
        *cell = false;
    }
    
    // Mark all current sand positions
    for (_, transform, _) in sand_query.iter() {
        if let Some((grid_x, grid_y)) = grid.world_to_grid(transform.translation.truncate()) {
            grid.set_occupied(grid_x, grid_y, true);
        }
    }
    
    // Process each grain of sand
    for (entity, mut transform, mut sand) in sand_query.iter_mut() {
        if sand.settled {
            continue;
        }
        
        // Get current grid position
        if let Some((grid_x, grid_y)) = grid.world_to_grid(transform.translation.truncate()) {
            // Clear current position
            grid.set_occupied(grid_x, grid_y, false);
            
            let mut new_x = grid_x;
            let mut new_y = grid_y;
            
            // Check if we can fall down (in grid coordinates, down is y+1)
            if grid_y + 1 < grid.height && !grid.is_occupied(grid_x, grid_y + 1) {
                new_y += 1;
            }
            // Check if we can fall diagonally
            else if grid_y + 1 < grid.height {
                let can_fall_left = grid_x > 0 && !grid.is_occupied(grid_x - 1, grid_y + 1);
                let can_fall_right = grid_x + 1 < grid.width && !grid.is_occupied(grid_x + 1, grid_y + 1);
                
                if can_fall_left && can_fall_right {
                    // Randomly choose left or right
                    if rng.gen_bool(0.5) {
                        new_x -= 1;
                    } else {
                        new_x += 1;
                    }
                    new_y += 1;
                } else if can_fall_left {
                    new_x -= 1;
                    new_y += 1;
                } else if can_fall_right {
                    new_x += 1;
                    new_y += 1;
                } else {
                    // Cannot move, settle
                    sand.settled = true;
                    grid.set_occupied(grid_x, grid_y, true);
                    continue;
                }
            } else {
                // Hit bottom of screen, settle
                sand.settled = true;
                grid.set_occupied(grid_x, grid_y, true);
                continue;
            }
            
            // Mark new position as occupied
            grid.set_occupied(new_x, new_y, true);
            
            // Update transform to new world position
            let new_world_pos = grid.grid_to_world(new_x, new_y);
            transform.translation.x = new_world_pos.x;
            transform.translation.y = new_world_pos.y;
        }
    }
}