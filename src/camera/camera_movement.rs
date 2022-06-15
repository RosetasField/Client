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
					let mut log_scale = projection.scale.ln();
					log_scale -= ev.y * 0.05;
					if log_scale < 9.0_f32.ln() {
						log_scale = 9.0_f32.ln();
					}
					if log_scale > 50.0_f32.ln() {
						log_scale = 50.0_f32.ln();
					}
					projection.scale = log_scale.exp();
				}
				MouseScrollUnit::Pixel => {
				}
			}
		}
	}
}

fn handle_mouse_position(
    mut query: Query<(&mut GameCamera, &mut Transform)>,
    mut windows: ResMut<Windows>
) {
    let win = windows.primary_mut();

    if win.is_focused() == false {
        return;
    }

    if win.cursor_position().is_none() {
        return;
    }

    let cursor_position = win.cursor_position().unwrap();

    let top: [Vec2; 2] = [Vec2::new(0.0, win.height() - (win.height() / 80.0)), Vec2::new(win.width(), win.height())];
    let bottom: [Vec2; 2] = [Vec2::new(0.0, 0.0), Vec2::new(win.width(), win.height() / 80.0)];
    let right: [Vec2; 2] = [Vec2::new(win.width() - (win.width() / 80.0), 0.0), Vec2::new(win.width(), win.height())];
    let left: [Vec2; 2] = [Vec2::new(0.0, 0.0), Vec2::new(win.width() / 80.0, win.height())];

    for (options, mut transform) in query.iter_mut() {
		println!("here");

		let (axis_v, axis_h) = if options.enabled {
			(
				movement_axis(cursor_position, right, left),
				movement_axis(cursor_position, bottom, top),
			)
		} else {
			(0.0, 0.0)
		};

		let (width_modifier, height_modifier) = 
		if win.width() > win.height() {
			(
				axis_v * (options.sensitivity),
				(((win.width() / win.height()) - 1.0) * (axis_h)) * (options.sensitivity),
			)
		}

		else if win.width() < win.height() {
			(
				(((win.height() / win.width()) - 1.0) * (axis_v)) * (options.sensitivity),
				axis_h * (options.sensitivity),
			)

		}
		
		else {
			(axis_v * options.sensitivity, axis_h * options.sensitivity)
		};

		transform.translation += Vec3::new(width_modifier, 0.0, height_modifier);
	}
}


pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
	fn build(&self, app: &mut App) {
		app
            .add_system(handle_mouse_position)
            .add_system(handle_mouse_scroll);
	}
}
