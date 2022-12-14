use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_log::{info, LogPlugin};

use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};

use ace_args;
use ace_game_shared::{protocol::Protocol, shared_config, Channels};
use ace_runtime::AgenticRuntimePlugin;

mod resources;
mod systems;

use systems::{events, init, tick};

fn main() {
    info!("Naia Bevy Server Demo starting up");

    // Build App
    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        .add_plugin(AgenticRuntimePlugin)
        // Args
        .init_resource::<ace_args::Agentic>()
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .run();
}
