use std::ops::Deref;
use std::sync::Arc;

use crate::config::ServerConfig;

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<InnerState>);

pub(crate) struct InnerState {
    pub(crate) config: ServerConfig,
}

impl InnerState {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }
}

impl Deref for AppState {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
