use bevy::prelude::*;

#[derive(Resource)]
pub struct SandGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub cell_size: f32,
}

impl SandGrid {
    pub fn new(width: usize, height: usize) -> Self {
        SandGrid {
            width,
            height,
            cells: vec![false; width * height],
            cell_size: 5.0, // Each cell is 5x5 pixels
        }
    }

    pub fn world_to_grid(&self, world_pos: Vec2) -> Option<(usize, usize)> {
        // Convert world coordinates to grid coordinates
        let half_width = (self.width as f32 * self.cell_size) / 2.0;
        let half_height = (self.height as f32 * self.cell_size) / 2.0;
        
        let grid_x = ((world_pos.x + half_width) / self.cell_size) as i32;
        // Flip y because in grid space, y=0 is top but in world space y increases upward
        let grid_y = ((half_height - world_pos.y) / self.cell_size) as i32;
        
        if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
            Some((grid_x as usize, grid_y as usize))
        } else {
            None
        }
    }

    pub fn grid_to_world(&self, grid_x: usize, grid_y: usize) -> Vec2 {
        let half_width = (self.width as f32 * self.cell_size) / 2.0;
        let half_height = (self.height as f32 * self.cell_size) / 2.0;
        
        Vec2::new(
            (grid_x as f32 * self.cell_size) - half_width + (self.cell_size / 2.0),
            // Flip y to convert from grid space to world space
            half_height - (grid_y as f32 * self.cell_size) - (self.cell_size / 2.0),
        )
    }
    
    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        match self.get_index(x, y) {
            Some(idx) => self.cells[idx],
            None => true, // Out of bounds is treated as occupied
        }
    }

    pub fn set_occupied(&mut self, x: usize, y: usize, occupied: bool) {
        if let Some(idx) = self.get_index(x, y) {
            self.cells[idx] = occupied;
        }
    }
}