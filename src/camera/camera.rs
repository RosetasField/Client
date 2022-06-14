use bevy::{prelude::*};

#[derive(Component)]
pub struct GameCamera {
	/// The speed the FlyCamera accelerates at. Defaults to `1.0`
	pub accel: f32,
	/// The maximum speed the FlyCamera can move at. Defaults to `0.5`
	pub max_speed: f32,
	/// The sensitivity of the FlyCamera's motion based on mouse movement. Defaults to `3.0`
	pub sensitivity: f32,
	/// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
	pub friction: f32,
	/// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub pitch: f32,
	/// The current pitch of the FlyCamera in degrees. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub yaw: f32,
	/// The current velocity of the FlyCamera. This value is always up-to-date, enforced by [FlyCameraPlugin](struct.FlyCameraPlugin.html)
	pub velocity: Vec3,
	/// If `false`, disable keyboard control of the camera. Defaults to `true`
	pub enabled: bool,
}
impl Default for GameCamera {
	fn default() -> Self {
		Self {
			accel: 1.5,
			max_speed: 0.5,
			sensitivity: 8.0,
			friction: 1.0,
			pitch: 0.0,
			yaw: 0.0,
			velocity: Vec3::ZERO,
			enabled: true,
		}
	}
}
