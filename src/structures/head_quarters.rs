use super::structures::{Structure, HeadQuarters};

use bevy::prelude::*;

impl Structure for HeadQuarters {
    fn build(
        &self,
        asset_server: &Res<AssetServer>,
        materials: &mut Assets<StandardMaterial>) -> PbrBundle {

        PbrBundle {
            mesh: asset_server.load("castle.obj"),
            material: materials.add(Color::rgb(0.8, 0.6, 0.0).into()),
            transform: Transform::from_xyz(self.x, self.y, self.z),
            ..default()
        }
    }

}