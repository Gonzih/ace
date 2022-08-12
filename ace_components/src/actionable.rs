use bevy::prelude::*;

#[derive(Component)]
pub struct Clickable {}

impl Clickable {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct Hoverable {}

impl Hoverable {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct Draggable {}

impl Draggable {
    pub fn new() -> Self {
        Self {}
    }
}
