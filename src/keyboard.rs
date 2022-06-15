use bevy::prelude::*;

use crate::states::*;

fn village_keyboard(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(GameState::StartMenu).unwrap();
    }
}

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
		app
            .add_system_set(SystemSet::on_update(GameState::Village).with_system(village_keyboard));
	}
}
