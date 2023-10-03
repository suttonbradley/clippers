use crate::store::ClipboardStore;
use crate::CLIP_STORE;

// TODO: make the op return a result
pub(crate) fn clip_store_op<F: Fn(&mut ClipboardStore)>(op: F) {
    unsafe { op(CLIP_STORE.get_mut().unwrap()) }
}
