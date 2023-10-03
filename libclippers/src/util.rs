use crate::store::ClipboardStore;
use crate::CLIP_STORE;

// TODO: make the op return a result
pub(crate) fn clip_store_op<T, F: Fn(&mut ClipboardStore) -> T>(op: F) -> T {
    unsafe { op(CLIP_STORE.get_mut().unwrap()) }
}
