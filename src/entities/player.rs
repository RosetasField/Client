use std::time::Instant;
use uuid::Uuid;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub unique_id: Uuid,
    pub xp: f32,
    pub creation_date: f32
}

impl Default for Player {

    fn default() -> Self {
    let start = Instant::now();

        Player {
            name: "RqndomUser".into(),
            unique_id: Uuid::new_v4(),
            xp: 0.0,
            creation_date: start.elapsed().as_secs_f32(),
        }
    }
}
