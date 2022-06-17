use bevy::prelude::*;

use crate::entities::*;

use crate::entities::structures::load_assets;
use crate::states::GameState;

pub struct VillageScenePlugin;

impl Plugin for VillageScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Village).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::Village).with_system(destroy));
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Button>>) {
    let mut i = 0.0;
    for ent in query.iter() {
        i += 1.0;
        commands.entity(ent).despawn_recursive();
    }
    println!("entities removed = {}", i);
}

fn setup_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<structures::CustomAssets>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection)>,
) {

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.9, 0.0, 0.2).into()),
        ..default()
    });

    for i in 1..100 {
        structures::spawn_head_quarters(
            i as f32 * 15.0,
            0.5,
            2.0,
            commands.spawn(), &assets, materials.as_mut());

        structures::spawn_mana_generator(
            i as f32 * 15.0,
            0.5,
            18.0,
            commands.spawn(), &assets, materials.as_mut());
    }

    commands.insert_resource(AmbientLight {
        color: Color::AQUAMARINE,
        brightness: 1.0,
    });

    for (mut transform, mut projection) in query.iter_mut() {
        transform.look_at(Vec3::new(15.0, 0.5, 0.0), Vec3::Y);
        transform.with_translation(Vec3::new(15.0, 60.0, 10.0));
        projection.scale = 50.0;
    }
}
