#[macro_use]
extern crate cfg_if;

pub mod cli;
pub mod config;

pub struct Agentic {
    pub args: cli::Args,
    pub config: config::Config,
}

fn read_file() -> Agentic {
    let args = cli::parse();
    let config = config::parse(&args.config_dir, &args.environment).expect(&format!(
        "Could not parse {} config from {}",
        args.environment, args.config_dir
    ));

    Agentic { args, config }
}

fn include_config() -> Agentic {
    let args: cli::Args = Default::default();
    let config = config::include();

    Agentic { args, config }
}

pub fn parse() -> Agentic {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            include_config()
        } else {
            read_file()
        }
    }
}

impl Default for Agentic {
    fn default() -> Self {
        parse()
    }
}
