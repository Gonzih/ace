use bevy::prelude::Component;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Position {
    pub x: Property<i16>,
    pub y: Property<i16>,
    pub z: Property<i16>,
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Position::new_complete(x, y, z)
    }
}
