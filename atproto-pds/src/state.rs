use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::config::ServerConfig;
use crate::storage::handle::HandleManager;

#[derive(Clone)]
pub(crate) struct AppState(pub Arc<InnerState>);

pub(crate) struct InnerState {
    pub(crate) config: ServerConfig,
    pub(crate) ready: AtomicBool,
    pub(crate) alive: AtomicBool,
    pub(crate) started: AtomicBool,
    pub(crate) handle_manager: Box<dyn HandleManager>,
}

impl InnerState {
    pub fn new(config: ServerConfig, handle_manager: Box<dyn HandleManager>) -> Self {
        let ready = AtomicBool::new(false);
        let alive = AtomicBool::new(false);
        let started = AtomicBool::new(false);
        Self {
            config,
            ready,
            alive,
            started,
            handle_manager,
        }
    }
}

impl Deref for AppState {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppState {
    pub fn set_ready(&self) {
        self.ready.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_alive(&self) {
        self.alive.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_started(&self) {
        self.started
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
