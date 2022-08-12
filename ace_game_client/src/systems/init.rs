use bevy::{
    ecs::system::*,
    log::info,
    prelude::{Transform, Vec3},
    render::camera::PerspectiveCameraBundle,
};

use naia_bevy_client::Client;

use ace_args::Agentic;
use ace_game_shared::{
    protocol::{Auth, Protocol},
    Channels,
};

use crate::resources::Global;

pub fn init(args: Res<Agentic>, mut commands: Commands, mut client: Client<Protocol, Channels>) {
    info!("ACE-Client demo started");

    let addr = &args.config.naia.advertise_addr();
    client.auth(Auth::new("donkey-boy"));
    client.connect(addr);

    info!("Connected to {} I guess?", addr);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.init_resource::<Global>();
}
