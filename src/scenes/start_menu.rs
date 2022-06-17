use bevy::prelude::*;

use crate::states::GameState;

use crate::uis::*;
use crate::entities::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_startup_system(menu::load_assets)

            .add_system_set(SystemSet::on_enter(GameState::StartMenu)
                .with_system(construct_background)
                .with_system(construct_buttons)
            )

            .add_system_set(SystemSet::on_exit(GameState::StartMenu)
                .with_system(destroy_background)
                .with_system(destroy_buttons)
            )

            .add_system_set(SystemSet::on_update(GameState::StartMenu)
                .with_system(menu::handle_start_button));
    }
}

fn construct_background(
    mut commands: Commands,
    assets: Res<decors::CustomAssets>

) {
    decors::spawn_background(commands.spawn(), &assets);
}

fn construct_buttons(
    mut commands: Commands,
    ui_assets: Res<menu::Assets>
) {
    menu::construct_start_button(commands.spawn(), &ui_assets);
}


fn destroy_buttons(mut commands: Commands, query: Query<Entity, With<Button>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn destroy_background(mut commands: Commands, query: Query<Entity, With<decors::Type>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
