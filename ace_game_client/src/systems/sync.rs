use bevy::{ecs::system::Query, transform::components::Transform};

use ace_game_shared::protocol::Position;

pub fn sync(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = f32::from(*pos.x);
        transform.translation.y = f32::from(*pos.y) * -1.0;
        transform.translation.z = f32::from(*pos.z);
    }
}
