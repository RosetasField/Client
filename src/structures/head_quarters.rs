use super::structures::{Structure, HeadQuarters};

use bevy::prelude::*;

impl Structure for HeadQuarters {
    fn build(
        &self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>) -> PbrBundle {

        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
            material: materials.add(Color::rgb(0.8, 0.6, 0.0).into()),
            transform: Transform::from_xyz(self.x, self.y, self.z),
            ..default()
        }
    }

}