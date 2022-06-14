use bevy::{prelude::*};

use super::camera::GameCamera;
use super::util::movement_axis;

fn forward_vector(rotation: &Quat) -> Vec3 {
	rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
	let f = forward_vector(rotation);
	let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
	f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
	// Rotate it 90 degrees to get the strafe direction
	Quat::from_rotation_y(90.0f32.to_radians())
		.mul_vec3(forward_walk_vector(rotation))
		.normalize()
}

fn handle_mouse_position(
	time: Res<Time>,
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

    for (mut options, mut transform) in query.iter_mut() {
		let (axis_h, axis_v) = if options.enabled {
			(
				movement_axis(cursor_position, right, left),
				movement_axis(cursor_position, bottom, top),
			)
		} else {
			(0.0, 0.0)
		};

		let rotation = transform.rotation;
		let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
			+ (forward_walk_vector(&rotation) * axis_v);
		let accel: Vec3 = if accel.length() != 0.0 {
			accel.normalize() * options.accel
		} else {
			Vec3::ZERO
		};

		let friction: Vec3 = if options.velocity.length() != 0.0 {
			options.velocity.normalize() * -1.0 * options.friction
		} else {
			Vec3::ZERO
		};

		options.velocity += accel * time.delta_seconds();

		// clamp within max speed
		if options.velocity.length() > options.max_speed {
			options.velocity = options.velocity.normalize() * options.max_speed;
		}

		let delta_friction = friction * time.delta_seconds();

		options.velocity = if (options.velocity + delta_friction).signum()
			!= options.velocity.signum()
		{
			Vec3::ZERO
		} else {
			options.velocity + delta_friction
		};

		transform.translation += options.velocity;
	}

}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
	fn build(&self, app: &mut App) {
		app
            .add_system(handle_mouse_position);
	}
}
