use bevy::prelude::*;

use crate::cameras::*;

pub fn spawn_camera(
    mut commands: Commands,
) {

    commands
        .spawn_bundle(OrthographicCameraBundle::new_3d())
        .insert(cameras::GameCamera::default());

    commands.spawn_bundle(UiCameraBundle::default());
}
