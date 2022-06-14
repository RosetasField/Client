use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::*};
use bevy_fly_camera::*;


mod structures;
use structures::structures::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy-introduction simple window".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Immediate,
            cursor_visible: false,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.9, 0.0, 0.2).into()),
        ..default()
    });

    commands.spawn_bundle(HeadQuarters {
        x: 2.0,
        y: 0.5,
        z: 2.0,
    }.build(meshes.as_mut(), materials.as_mut()));

    commands.spawn_bundle(ManaGenerator {
        x: 5.0,
        y: 0.5,
        z: 6.0,
    }.build(meshes.as_mut(), materials.as_mut()));

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle::new_3d())
        .insert(FlyCamera::default());
}
