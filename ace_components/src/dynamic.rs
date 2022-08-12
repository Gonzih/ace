use bevy::prelude::*;

#[derive(Component)]
pub struct Dynamic {
    pub code: &'static str,
    pub init: bool,
}

impl Dynamic {
    pub fn test_timer() -> Self {
        let code = include_str!("js/test_timer.js");
        let init = false;

        Dynamic { code, init }
    }
}
