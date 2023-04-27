use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::configuration::Configuration;
use crate::storage::handle::HandleManager;

#[derive(Clone)]
pub(crate) struct Context(pub Arc<InnerContext>);

pub(crate) struct InnerContext {
    pub(crate) version: String,
    pub(crate) configuration: Configuration,
    pub(crate) ready: AtomicBool,
    pub(crate) alive: AtomicBool,
    pub(crate) started: AtomicBool,
    pub(crate) handle_manager: Box<dyn HandleManager>,
}

impl InnerContext {
    pub fn new(
        version: String,
        configuration: Configuration,
        handle_manager: Box<dyn HandleManager>,
    ) -> Self {
        let ready = AtomicBool::new(false);
        let alive = AtomicBool::new(false);
        let started = AtomicBool::new(false);
        Self {
            version,
            configuration,
            ready,
            alive,
            started,
            handle_manager,
        }
    }
}

impl Deref for Context {
    type Target = InnerContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Context {
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
