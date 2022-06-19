use bevy::{prelude::*, ui::FocusPolicy};
use bevy::ecs::system::EntityCommands;

pub struct Assets {
    pub font: Handle<Font>,
    pub gold_icon: Handle<Image>,
    pub mana_icon: Handle<Image>,
    pub grimoire_icon: Handle<Image>,
}

pub fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let ui_assets = Assets {
        font: assets.load("IMMORTAL.ttf"),
        gold_icon: assets.load("gold_icon.png"),
        mana_icon: assets.load("mana_icon.png"),
        grimoire_icon: assets.load("grimoire_icon.png"),
    };

    commands.insert_resource(ui_assets);
}

pub fn construct_gold_infos(
    mut commands: EntityCommands,
    assets: &Res<Assets>
) {
    commands
        .insert_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(5.0)),
                align_items: AlignItems::FlexStart,
                position: Rect {
                    bottom: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgba(1.0, 1.0, 0.0, 0.01).into(),
            ..default()
        })

        .with_children(|parent| {
            parent.spawn_bundle( ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                    ..default()
                },
                image: assets.gold_icon.clone().into(),
                ..default()
            })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                    ..default()
                },
                text: Text::with_section(
                    "5 239 292",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default()),
                ..default()
            });
        });
    });
}

pub fn construct_player_infos(
    commmands: EntityCommands,
    assets: &Res<Assets>
) {

}
