use bevy::prelude::*;

fn is_contained(area: [Vec2; 2], position: Vec2) -> bool {
    if position.x < area[0].x || position.y < area[0].y {
        return false;
    }
    return position.x <= area[1].x && position.y <= area[1].y;
}

pub fn movement_axis(
	position: Vec2,
	positive: [Vec2; 2],
    negative: [Vec2; 2],
) -> f32 {
	let mut axis = 0.0;

    if is_contained(positive, position) {
        axis += 1.0;
    }

    if is_contained(negative, position) {
        axis -= 1.0;
    }
    axis
}
