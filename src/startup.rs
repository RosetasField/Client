use bevy::prelude::*;

use crate::cameras::{menu_camera::menu_camera::*, game_camera::game_camera::*};

pub fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 50.0;
    camera.transform = Transform::from_xyz(15.0, 60.0, 10.0).looking_at(Vec3::new(15.0, 0.5, 0.0), Vec3::Y);

    commands
    .spawn_bundle(camera)
    .insert(GameCamera::default())
    .insert(MainCamera);

    commands.spawn_bundle(UiCameraBundle::default());

}
