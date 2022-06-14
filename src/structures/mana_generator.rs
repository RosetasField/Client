use super::structures::{Structure, ManaGenerator};

use bevy::prelude::*;

impl Structure for ManaGenerator {
    fn build(
        &self,
        asset_server: &Res<AssetServer>,
        materials: &mut Assets<StandardMaterial>) -> PbrBundle {

        PbrBundle {
            mesh: asset_server.load("castle.obj"),
            material: materials.add(Color::rgb(148.0 / 255.0, 0.0, 211.0 / 255.0).into()),
            transform: Transform::from_xyz(self.x, self.y, self.z),
            ..default()
        }
    }

}