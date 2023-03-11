use anyhow::anyhow;
use atproto_core::jwk::jwks_from;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clap::{Parser, Subcommand};

mod config;
mod server;
mod state;
mod storage;
mod util;

use atproto_core::error::{AtProtoError, Result};

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

        #[arg(
            short,
            env = "SIGNING_KEYS",
            use_value_delimiter = true,
            value_delimiter = ',',
            help = "One or more comma separated values of file paths to signing keys."
        )]
        signing_keys: Vec<String>,

        #[arg(
            short,
            env = "DEFAULT_SIGNING_KEY",
            help = "The KID of a signing key to be used as the default signing key."
        )]
        default_signing_key: Option<String>,

        #[arg(short, env = "HANDLE_MANAGER", default_value = "memory")]
        handle_manager_type: String,
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
        Commands::Server {
            port,
            address,
            signing_keys,
            default_signing_key,
            handle_manager_type,
        } => {
            println!("signing_keys {:?}", signing_keys);
            let jwks = jwks_from(signing_keys).unwrap_or(vec![]);
            if jwks.is_empty() {
                return Err(AtProtoError {
                    err: anyhow!("at least one signing key must be provided"),
                });
            }

            let actual_default_signing_key: Result<jsonwebtoken::jwk::Jwk, AtProtoError> =
                match default_signing_key {
                    None => Ok(jwks[0].clone()),
                    Some(default_signing_key_kid) => {
                        let found_signing_key = jwks
                            .iter()
                            .find(|jwk| jwk.common.key_id == Some(default_signing_key_kid.clone()));
                        if found_signing_key.is_none() {
                            return Err(AtProtoError {
                                err: anyhow!("at least one signing key must be provided"),
                            });
                        }
                        Ok(found_signing_key.unwrap().clone())
                    }
                };
            if actual_default_signing_key.is_err() {
                return Err(actual_default_signing_key.err().unwrap());
            }

            let server_config = ServerConfig {
                version,
                port,
                address,
                signing_keys: jwks.clone(),
                default_signing_key: actual_default_signing_key.unwrap(),
                handle_manager_type: handle_manager_type,
            };
            cmd_server(server_config).await?;
        }
    }

    Ok(())
}
