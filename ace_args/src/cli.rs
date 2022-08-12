use clap::Parser;

/// Agentic Backend CLI
#[derive(Parser, Debug)]
pub struct Args {
    /// config directory
    #[clap(short, long, value_parser, default_value = "config")]
    pub config_dir: String,
    /// environment (production/development/staging)
    #[clap(short, long, value_parser, default_value = "development")]
    pub environment: String,
}

pub fn parse() -> Args {
    Args::parse()
}

impl Default for Args {
    fn default() -> Self {
        Args {
            config_dir: "config".to_string(),
            environment: env!("ACE_ENV").to_string(),
        }
    }
}
