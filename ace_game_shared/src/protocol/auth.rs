use bevy::prelude::Component;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Auth {
    pub cookie: Property<String>,
}

impl Auth {
    pub fn new(cookie: &str) -> Self {
        Auth::new_complete(cookie.to_string())
    }
}
