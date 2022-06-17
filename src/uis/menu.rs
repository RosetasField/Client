use bevy::{prelude::*, ui::FocusPolicy};
use bevy::ecs::system::EntityCommands;

use crate::states::*;

#[derive(Component)]
pub struct ButtonActive(pub bool);

pub struct Assets {
    pub font: Handle<Font>,
    pub button: Handle<Image>,
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let ui_assets = Assets {
        font: assets.load("IMMORTAL.ttf"),
        button: assets.load("button.png")
    };

    commands.insert_resource(ui_assets);
}

pub fn construct_start_button(
    mut commands: EntityCommands,
    ui_assets: &Res<Assets>
) {
    commands
    .insert_bundle(ButtonBundle {
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

    .insert(ButtonActive(true))
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

pub fn handle_start_button(
    mut query: Query<(&mut ButtonActive, &Interaction), Changed<Interaction>>,
    mut app_state: ResMut<State<GameState>>
) {
    for (mut active, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if active.0 {
                    app_state.set(GameState::Village).unwrap();
                    active.0 = false;
                }
            }
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}
