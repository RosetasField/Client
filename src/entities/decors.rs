use bevy::{prelude::*, ui::FocusPolicy};
use bevy::ecs::system::EntityCommands;

pub struct CustomAssets {
    background: Handle<Image>,
}

#[derive(Component)]
pub enum Type {
    Background,
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(CustomAssets {
        background: assets.load("menu_background.png"),
    });
}

pub fn spawn_background(
    mut commands: EntityCommands,
    assets: &Res<CustomAssets>,
) {
        commands
            .insert_bundle(ImageBundle {

                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                image: assets.background.clone().into(),
                ..default()
            })
            .insert(FocusPolicy::Pass)
            .insert(Type::Background);
}
