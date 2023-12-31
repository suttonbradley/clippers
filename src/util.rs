use crate::store::ClipboardStore;
use crate::CLIP_STORE;

// TODO: make the op return a result
pub(crate) fn clip_store_op<F: Fn(&mut ClipboardStore)>(op: F) {
    loop {
        if let Some(store) = unsafe { CLIP_STORE.get_mut() } {
            op(store);
            break;
        }
    }
}
