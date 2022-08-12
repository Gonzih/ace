use bevy::{app::App, DefaultPlugins};

use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin, Stage};

use ace_args;
use ace_game_shared::{protocol::Protocol, shared_config, Channels};

use crate::systems::{events, init, input, sync, tick};

pub fn run() {
    App::default()
        //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        //Startup Systems
        .add_startup_system(init)
        // Args
        .init_resource::<ace_args::Agentic>()
        //Realtime Gameplay Loop
        .add_system_to_stage(Stage::Connection, events::connect_event)
        .add_system_to_stage(Stage::Disconnection, events::disconnect_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::spawn_entity_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::insert_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::update_component_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::Frame, input)
        .add_system_to_stage(Stage::PostFrame, sync)
        //Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        .run();
}
