use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;

pub struct CustomAssets {
    castle: Handle<Mesh>,
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(CustomAssets {
        castle: assets.load("castle.obj"),
    });
}

#[derive(Component)]
pub enum Type {
    HeadQuarters,
    ManaGenerator,
}

pub fn spawn_head_quarters(
    x: f32,
    y: f32,
    z: f32,
    mut commands: EntityCommands,
    assets: &Res<CustomAssets>,
    materials: &mut Assets<StandardMaterial>
) {
        commands
        .insert_bundle(PbrBundle {
            mesh: assets.castle.clone(),
            material: materials.add(Color::rgb(0.8, 0.6, 0.0).into()),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        })
        .insert(Type::HeadQuarters);
}

pub fn spawn_mana_generator(
    x: f32,
    y: f32,
    z: f32,
    mut commands: EntityCommands,
    assets: &Res<CustomAssets>,
    materials: &mut Assets<StandardMaterial>
) {
        commands
        .insert_bundle(PbrBundle {
            mesh: assets.castle.clone(),
            material: materials.add(Color::rgb(148.0 / 255.0, 0.0, 211.0 / 255.0).into()),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        })
        .insert(Type::ManaGenerator);
}
