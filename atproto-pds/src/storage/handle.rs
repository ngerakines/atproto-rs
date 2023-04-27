use anyhow::Result;
use async_trait::async_trait;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;

use super::types::Handle;

#[async_trait]
pub trait HandleManager: Sync + Send {
    async fn set(&self, handle: Handle) -> Result<()>;
    async fn get(&self, did: String) -> Result<Option<Handle>>;
}

#[cfg(debug_assertions)]
pub struct NullHandleManager;

#[cfg(debug_assertions)]
impl Default for NullHandleManager {
    fn default() -> Self {
        NullHandleManager
    }
}

#[cfg(debug_assertions)]
#[async_trait]
impl HandleManager for NullHandleManager {
    async fn set(&self, _: Handle) -> Result<()> {
        Ok(())
    }
    async fn get(&self, _: String) -> Result<Option<Handle>> {
        Ok(None)
    }
}

#[derive(Default)]
struct InnerMemoryHandleManager {
    handles: HashMap<String, Handle>,
}

#[derive(Default)]
pub struct MemoryHandleManager {
    inner: Mutex<RefCell<InnerMemoryHandleManager>>,
}

#[async_trait]
impl HandleManager for MemoryHandleManager {
    async fn set(&self, handle: Handle) -> Result<()> {
        let inner_lock = self.inner.lock();
        let mut inner = inner_lock.borrow_mut();

        let did = handle.did.clone();
        inner.handles.insert(did, handle.clone());

        Ok(())
    }

    async fn get(&self, did: String) -> Result<Option<Handle>> {
        let inner_lock = self.inner.lock();
        let inner = inner_lock.borrow();

        match inner.handles.get(&did) {
            Some(val_ref) => Ok(Some(val_ref.clone())),
            None => Ok(None),
        }
    }
}

pub fn get_handle_manager(handle_manager_type: &str) -> Box<dyn HandleManager> {
    match handle_manager_type {
        #[cfg(debug_assertions)]
        "null" => Box::<NullHandleManager>::default() as Box<dyn HandleManager>,

        "memory" => Box::<MemoryHandleManager>::default() as Box<dyn HandleManager>,

        _ => panic!("Unknown handle manager type: {handle_manager_type}"),
    }
}
