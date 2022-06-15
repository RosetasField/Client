use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use bevy_obj::*;

mod states;
use scenes::village::VillageScenePlugin;
use states::GameState;

mod cameras;
use cameras::game_camera::camera_movement::*;

mod startup;
use startup::*;

mod scenes;
use scenes::start_menu::MainMenuPlugin;

mod keyboard;
use keyboard::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roseta's Field | [Indev]".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Fifo,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })

        .add_state(GameState::StartMenu)

        .add_startup_system(spawn_camera)
        .add_system(ui_example)

        .add_plugins(DefaultPlugins)
        .add_plugin(KeyboardPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(VillageScenePlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(ObjPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}