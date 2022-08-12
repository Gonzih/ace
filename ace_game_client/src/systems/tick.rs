use bevy::ecs::system::{Query, ResMut};

use naia_bevy_client::Client;

use ace_game_shared::{
    behavior as shared_behavior,
    protocol::{Position, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn tick(
    mut global: ResMut<Global>,
    mut client: Client<Protocol, Channels>,
    mut position_query: Query<&mut Position>,
) {
    //on tick, do game logic

    if let Some(command) = global.queued_command.take() {
        if let Some(predicted_entity) = global
            .owned_entity
            .as_ref()
            .map(|owned_entity| owned_entity.predicted)
        {
            if let Some(client_tick) = client.client_tick() {
                if global.command_history.can_insert(&client_tick) {
                    //record command into global command history
                    global.command_history.insert(client_tick, command.clone());

                    //Send command thru playercommand channel
                    client.send_message(Channels::PlayerCommand, &command);

                    if let Ok(mut position) = position_query.get_mut(predicted_entity) {
                        shared_behavior::process_command(&command, &mut position);
                    }
                }
            }
        }
    }
}
