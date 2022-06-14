use super::structures::{Structure, ManaGenerator};

use bevy::prelude::*;

impl Structure for ManaGenerator {
    fn build(
        &self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>) -> PbrBundle {

        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus { 
                radius: 3.0,
                ..default()
            })),
            material: materials.add(Color::rgb(148.0 / 255.0, 0.0, 211.0 / 255.0).into()),
            transform: Transform::from_xyz(self.x, self.y, self.z),
            ..default()
        }
    }

}