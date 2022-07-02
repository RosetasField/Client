use bevy::{prelude::*, ui::FocusPolicy, render::camera::RenderTarget, window::WindowId};

pub struct Assets {
    pub font: Handle<Font>,
    pub gold_icon: Handle<Image>,
    pub mana_icon: Handle<Image>,
    pub grimoire_icon: Handle<Image>,
}

#[derive(Component)]
pub struct ResourcesInfo;

#[derive(Component)]
pub struct MiniMap;

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

pub fn construct_resources_infos(
    mut commands: Commands,
    assets: Res<Assets>
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(9.0)),
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                padding: Rect {left: Val::Percent(1.0), ..default()},
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
        .insert(ResourcesInfo)
        .with_children(|parent| {
            construct_gold_infos(parent, &assets)})
        .with_children(|parent| {
            construct_mana_infos(parent, &assets)})
        .with_children(|parent| {
            construct_grimoire_infos(parent, &assets)});
}

pub fn construct_mini_map(
    mut commands: Commands,
) {
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0/6.0), Val::Percent(100.0/6.0)),
                ..default()
            },
            color: Color::rgba(1.0, 1.0, 0.0, 0.01).into(),
            ..default()
        })
        .insert(MiniMap);
}

pub fn construct_gold_infos(
    parent: &mut ChildBuilder,
    assets: &Res<Assets>
) {

    parent.spawn_bundle(
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0/6.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::rgba(1.0, 1.0, 0.0, 0.01).into(),
                ..default()
            })

        .with_children(|parent| {
            parent.spawn_bundle( ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    ..default()
                },
                image: assets.gold_icon.clone().into(),
                ..default()
            });
        })

        .insert(FocusPolicy::Pass)

        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
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
}

pub fn construct_mana_infos(
    parent: &mut ChildBuilder,
    assets: &Res<Assets>
) {

    parent.spawn_bundle(
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0/6.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::rgba(1.0, 1.0, 0.0, 0.01).into(),
                ..default()
            })

        .with_children(|parent| {
            parent.spawn_bundle( ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    ..default()
                },
                image: assets.mana_icon.clone().into(),
                ..default()
            });
        })

        .insert(FocusPolicy::Pass)

        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    ..default()
                },
                text: Text::with_section(
                    "2 292",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default()),
                ..default()
            });
        });
}

pub fn construct_grimoire_infos(
    parent: &mut ChildBuilder,
    assets: &Res<Assets>
) {

    parent.spawn_bundle(
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0/6.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::rgba(1.0, 1.0, 0.0, 0.01).into(),
                ..default()
            })

        .with_children(|parent| {
            parent.spawn_bundle( ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    ..default()
                },
                image: assets.grimoire_icon.clone().into(),
                ..default()
            });
        })

        .insert(FocusPolicy::Pass)

        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    ..default()
                },
                text: Text::with_section(
                    "95%",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default()),
                ..default()
            });
        });
}
