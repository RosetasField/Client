use bevy::{prelude::*, ui::FocusPolicy};

use crate::states::GameState;

#[derive(Component)]
pub struct ButtonActive(bool);

struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_assets)
            .add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(GameState::StartMenu).with_system(destroy))
            .add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(handle_start_button));
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Button>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn handle_start_button(
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

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let ui_assets = UiAssets {
        font: assets.load("IMMORTAL.ttf"),
        button: assets.load("button.png")
    };

    commands.insert_resource(ui_assets);
}

fn setup_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>
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
