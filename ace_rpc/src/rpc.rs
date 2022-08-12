use ace_args::config::Nats;
use nats::asynk::{Connection, Options};

pub(crate) async fn connect() -> std::io::Result<Connection> {
    let args = ace_args::parse();

    let Nats {
        host,
        port,
        username,
        password,
    } = args.config.nats;

    Options::with_user_pass(&username, &password)
        .connect(&format!("{}:{}", host, port))
        .await
}
