use bevy::{prelude::*, input::mouse::*};

use super::game_camera::GameCamera;
use super::util::movement_axis;

fn handle_mouse_scroll(
	mut query: Query<&mut OrthographicProjection>,
	mut scroll_evr: EventReader<MouseWheel>,
	mut windows: ResMut<Windows>
) {
	let win = windows.primary_mut();

    if win.is_focused() == false {
        return;
    }

	for mut projection in query.iter_mut() {

		for ev in scroll_evr.iter() {
			match ev.unit {
				MouseScrollUnit::Line => {
					let mut log_scale = projection.scale;
					log_scale -= ev.y * 1.05;
					if log_scale < 9.0 {
						log_scale = 9.0;
					}
					if log_scale > 50.0 {
						log_scale = 50.0;
					}
					projection.scale = log_scale;
				}
				MouseScrollUnit::Pixel => {
				}
			}
		}
	}
}

fn handle_mouse_position(
    mut query: Query<&mut GameCamera>,
    mut windows: ResMut<Windows>,
	mut cursor_evt: EventReader<CursorMoved>
) {
    let win = windows.primary_mut();

    if win.is_focused() == false {
        return;
    }

    if win.cursor_position().is_none() {
        return;
    }

	let top: [Vec2; 2] = [Vec2::new(0.0, win.height() - (win.height() / 80.0)), Vec2::new(win.width(), win.height())];
    let bottom: [Vec2; 2] = [Vec2::new(0.0, 0.0), Vec2::new(win.width(), win.height() / 80.0)];
    let right: [Vec2; 2] = [Vec2::new(win.width() - (win.width() / 80.0), 0.0), Vec2::new(win.width(), win.height())];
    let left: [Vec2; 2] = [Vec2::new(0.0, 0.0), Vec2::new(win.width() / 80.0, win.height())];

	if cursor_evt.is_empty() {
		return;
	}

	let cursor = cursor_evt.iter().last().unwrap();

	for mut options in query.iter_mut() {

		let (axis_v, axis_h) = if options.enabled {
			(
				movement_axis(cursor.position, right, left),
				movement_axis(cursor.position, bottom, top),
			)
		} else {
			(0.0, 0.0)
		};

		options.vel = Vec2::new(axis_v * options.sensitivity, axis_h * options.sensitivity);
	}

}

fn move_camera(
    mut query: Query<(&mut GameCamera, &mut Transform)>,
    mut windows: ResMut<Windows>,
) {

    let win = windows.primary_mut();

    if win.is_focused() == false {
        return;
    }

	for (options, mut transform) in query.iter_mut() {
		if !options.enabled || options.vel == Vec2::ZERO {
			return;
		}

		let (width_modifier, height_modifier) = 
		if win.width() > win.height() {
			(
				options.vel.x,
				((win.width() / win.height()) - 1.0) * (options.vel.y),
			)
		}

		else if win.width() < win.height() {
			(
				((win.height() / win.width()) - 1.0) * (options.vel.x),
				options.vel.y,
			)

		}
		
		else {
			(options.vel.x * options.sensitivity, options.vel.y * options.sensitivity)
		};

		transform.translation += Vec3::new(width_modifier, 0.0, height_modifier);
	}
}



pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
	fn build(&self, app: &mut App) {
		app
            .add_system(handle_mouse_position)
            .add_system(move_camera)
            .add_system(handle_mouse_scroll);
	}
}
