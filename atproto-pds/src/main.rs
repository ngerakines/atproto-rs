use anyhow::anyhow;
use atproto_core::jwk::jwks_from;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clap::{Parser, Subcommand};

mod config;
mod server;
mod state;
mod util;

use atproto_core::error::{Result, AtProtoError};

use crate::config::ServerConfig;
use crate::server::cmd_server;

#[derive(Debug, Parser)]
#[command(name = "atproto-pds")]
#[command(about = "An atprotocol PDS server and tools", long_about = None, disable_colored_help=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "server", about = "Starts the PDS server", long_about = None)]
    Server {
        #[arg(short, env = "PORT", default_value = "8080")]
        port: u16,

        #[arg(short, env = "ADDRESS", default_value = "0.0.0.0")]
        address: String,

        #[arg(short, env = "SIGNING_KEYS", use_value_delimiter = true, value_delimiter = ',')]
        signing_keys: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), AtProtoError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "atproto_pds=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(debug_assertions)]
    warn!("Debug assertions enabled");

    let version = option_env!("GIT_HASH")
        .unwrap_or(env!("CARGO_PKG_VERSION", "develop"))
        .to_string();

    let args = Cli::parse();

    match args.command {
        Commands::Server { port, address, signing_keys } => {
            println!("signing_keys {:?}", signing_keys);
            let jwks = jwks_from(signing_keys).unwrap_or(vec![]);
            if jwks.is_empty() {
                return Err(AtProtoError{ err: anyhow!("at least one signing key must be provided")});
            }

            let server_config = ServerConfig {
                version,
                port,
                address,
                signing_keys: jwks.clone(),
            };
            cmd_server(server_config).await?;
        }
    }

    Ok(())
}
