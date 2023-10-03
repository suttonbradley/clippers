mod store;
mod util;
#[cfg(feature = "listener")]
mod win;

#[cfg(feature = "listener")]
use std::thread::{self, JoinHandle};

use log::trace;
use store::ClipboardStore;

pub(crate) static mut CLIP_STORE: std::sync::OnceLock<ClipboardStore> = std::sync::OnceLock::new();

fn init_clipboard() {
    // Create ClipboardStore and save in OnceLock
    unsafe {
        if let Err(_) = CLIP_STORE.set(ClipboardStore::new()) {
            panic!("Failed to setup ClipboardStore");
        }
    }
    trace!("Set up clipboard OnceLock");
}

#[cfg(not(feature = "listener"))]
pub fn init() {
    init_clipboard();
}

#[cfg(feature = "listener")]
pub fn init() -> JoinHandle<()> {
    init_clipboard();

    // Run windows listener loop
    unsafe {
        crate::win::init();
        trace!("Starting wndproc message receiver loop...");
        thread::spawn(|| crate::win::run_loop())
    }
}

// TODO: probably not the most ergonomic choice
pub fn get_matches(query: &str) {
    util::clip_store_op(|store| store.get_matches(query));
}
