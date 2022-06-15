use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera {
	/// The sensitivity of the GameCameras's motion based on mouse movement. Defaults to `1.0`
	pub sensitivity: f32,
	/// The current pitch of the GameCameras in degrees.
	pub pitch: f32,
	/// The current pitch of the GameCameras in degrees.
	pub yaw: f32,
	/// If `false`, disables control of the camera. Defaults to `true`
	pub enabled: bool,
}
impl Default for GameCamera {
	fn default() -> Self {
		Self {
			sensitivity: 1.0,
			pitch: 0.0,
			yaw: 0.0,
			enabled: true,
		}
	}
}
