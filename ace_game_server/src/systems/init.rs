use std::collections::HashMap;

use bevy_ecs::system::*;
use bevy_log::info;

use naia_bevy_server::{Server, ServerAddrs};

use ace_args::Agentic;
use ace_game_shared::{protocol::Protocol, Channels};

use crate::resources::Global;

pub fn init(args: Res<Agentic>, mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("Ace Server Demo is starting");

    // WebRTC focus here, fon now it makes sense since web first
    let addr = &args.config.naia.advertise_addr_webrtc;

    // Naia Server initialization
    let server_addresses = ServerAddrs::new(
        format!("0.0.0.0:{}", args.config.naia.webrtc_port)
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        format!("0.0.0.0:{}", args.config.naia.udp_port)
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        addr,
    );

    info!("Starting server on {}", addr);

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Resources
    commands.insert_resource(Global {
        main_room_key,
        user_to_prediction_map: HashMap::new(),
        player_last_command: HashMap::new(),
    })
}
