use bevy::prelude::*;

#[path = "structures/mod.rs"]
mod structures;

use structures::structures::*;

#[path = "camera/mod.rs"]
mod camera;
use crate::camera::game_camera::*;

#[path = "entities/mod.rs"]
mod entities;
use crate::entities::player::Player;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    commands.spawn().insert(
        Player {
            name: String::from("RqndomHax"),
            ..default()
        });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.9, 0.0, 0.2).into()),
        ..default()
    });

    for i in 1..100 {
        commands.spawn_bundle(HeadQuarters {
            x: i as f32 * 15.0,
            y: 0.5,
            z: 2.0,
        }.build(&asset_server, materials.as_mut()));
    
        commands.spawn_bundle(ManaGenerator {
            x: i as f32 * 15.0,
            y: 0.5,
            z: 18.0,
        }.build(&asset_server, materials.as_mut()));
    }

    commands.insert_resource(AmbientLight {
        color: Color::AQUAMARINE,
        brightness: 1.0,
    });

    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 900.0;
    camera.transform = Transform::from_xyz(15.0, 60.0, 10.0).looking_at(Vec3::new(15.0, 0.5, 0.0), Vec3::Y);

    commands
        .spawn_bundle(camera)
        .insert(GameCamera::default());

}
