use bevy::pbr::LightEntity;
use bevy::prelude::*;

use crate::cameras::cameras::GameCamera;
use crate::entities::*;

use crate::uis::*;

use crate::states::GameState;

pub struct VillageScenePlugin;

impl Plugin for VillageScenePlugin {
    fn build(&self, app: &mut App) {
        app

            .add_startup_system(village::load_assets)

            .add_system_set(SystemSet::on_enter(GameState::Village)
                .with_system(reset_camera)
                .with_system(construct)

                .with_system(village::construct_resources_infos)

                .with_system(village::construct_mini_map)
            )

            .add_system_set(SystemSet::on_exit(GameState::Village)
                .with_system(destroy_structures)
                .with_system(destroy_lights)
                .with_system(destroy_ui)
            );
    }
}

fn reset_camera(
    mut query: Query<(&mut Transform, (&mut OrthographicProjection, With<GameCamera>))>,
) {

    for (mut transform, mut projection) in query.iter_mut() {
        transform.with_translation(Vec3::new(15.0, 60.0, 10.0));
        projection.0.scale = 50.0;
        transform.translation = Vec3::new(15.0, 60.0, 10.0);
        transform.look_at(Vec3::new(15.0, 0.5, 0.0), Vec3::Y);
    }
}

fn construct(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<structures::CustomAssets>,
) {

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

}

fn destroy_structures(mut commands: Commands, query: Query<Entity, With<structures::Type>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn destroy_lights(mut commands: Commands, query: Query<Entity, With<LightEntity>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<village::ResourcesInfo>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
