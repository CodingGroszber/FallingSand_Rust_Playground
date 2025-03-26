use bevy::prelude::*;

#[derive(Component)]
pub struct Sand {
    pub velocity: Vec2,
    pub settled: bool,
}

impl Default for Sand {
    fn default() -> Self {
        Sand {
            velocity: Vec2::new(0.0, -1.0),
            settled: false,
        }
    }
}