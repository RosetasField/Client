use bevy::{prelude::*, ui::FocusPolicy};

use crate::states::GameState;

use crate::uis::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_startup_system(menu::load_assets)

            .add_system_set(SystemSet::on_enter(GameState::StartMenu)
                .with_system(setup))

            .add_system_set(SystemSet::on_exit(GameState::StartMenu)
                .with_system(destroy))

            .add_system_set(SystemSet::on_update(GameState::StartMenu)
                .with_system(menu::handle_start_button));
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Button>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    ui_assets: Res<menu::Assets>
) {

    commands
    .spawn_bundle(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            margin: Rect::all(Val::Auto),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
    .insert(menu::ButtonActive(true))
    .with_children(|parent| {
        parent.spawn_bundle( ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            image: ui_assets.button.clone().into(),
            ..default()
        })
            .insert(FocusPolicy::Pass)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Play",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 90.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default()),
                    ..default()
                });
            });
    });

}
