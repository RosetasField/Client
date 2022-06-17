use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub unique_id: Uuid,
    pub xp: f32,
    pub creation_date: u128
}

impl Default for Player {

    fn default() -> Self {

        let time: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_millis(),
            Err(_) => 0,
        };

        Player {
            name: "RqndomUser".into(),
            unique_id: Uuid::new_v4(),
            xp: 0.0,
            creation_date: time,
        }
    }
}
