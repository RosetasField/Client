use bevy::prelude::*;

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
