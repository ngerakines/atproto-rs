use std::env;

use anyhow::Result;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod configuration;
mod context;
mod server;
mod storage;
mod util;

use crate::{configuration::Configuration, server::cmd_server};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "atproto_pds=debug,atproto_core=debug,info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    #[cfg(debug_assertions)]
    warn!("Debug assertions enabled");

    let configuration = Configuration::new()?;

    let version = option_env!("GIT_HASH")
        .unwrap_or(env!("CARGO_PKG_VERSION", "develop"))
        .to_string();

    cmd_server(version, configuration).await?;
    Ok(())
}
