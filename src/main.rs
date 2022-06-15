use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::*};

use bevy_obj::*;

mod camera;
use camera::camera_movement::GameCameraPlugin;

mod startup;
use startup::*;

mod entities;
use entities::player::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "WizBattle | DEV".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Fifo,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_display)
        .add_plugin(ObjPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn player_display(
	query: Query<&Player>,
) {

	for player in query.iter() {
        println!("player name = {} | player xp = {} | player uuid = {} | player creation_date = {}",
            player.name, player.xp, player.unique_id, player.creation_date);
	}
}