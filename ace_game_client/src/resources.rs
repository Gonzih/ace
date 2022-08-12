use std::default::Default;

use bevy::ecs::entity::Entity;

use naia_bevy_client::CommandHistory;

use ace_game_shared::protocol::KeyCommand;

use std::collections::VecDeque;

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
        OwnedEntity {
            confirmed: confirmed_entity,
            predicted: predicted_entity,
        }
    }
}

pub struct Global {
    pub owned_entity: Option<OwnedEntity>,
    pub queued_command: Option<KeyCommand>,
    pub command_history: CommandHistory<KeyCommand>,
}

//default Derive was acting busted on command_history, so manually implemented here.

impl Default for Global {
    fn default() -> Self {
        Global {
            owned_entity: None,
            queued_command: None,
            command_history: CommandHistory::<KeyCommand>::new(),
        }
    }
}
