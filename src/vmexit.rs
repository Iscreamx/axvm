use spin::RwLock;

/// Callback type invoked after a vCPU exits from guest context.
pub type VmExitHandler = fn(vm_id: u32);

static VMEXIT_HANDLER: RwLock<Option<VmExitHandler>> = RwLock::new(None);

/// Register a callback that observes guest VM-exit events.
pub fn register_vmexit_handler(handler: VmExitHandler) {
    *VMEXIT_HANDLER.write() = Some(handler);
}

pub(crate) fn notify_vmexit(vm_id: u32) {
    let handler = *VMEXIT_HANDLER.read();
    if let Some(handler) = handler {
        handler(vm_id);
    }
}

#[cfg(test)]
pub(crate) fn clear_vmexit_handler_for_test() {
    *VMEXIT_HANDLER.write() = None;
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicU32, Ordering};

    use super::{clear_vmexit_handler_for_test, notify_vmexit, register_vmexit_handler};

    static LAST_VMEXIT_VM_ID: AtomicU32 = AtomicU32::new(0);

    fn record_vmexit(vm_id: u32) {
        LAST_VMEXIT_VM_ID.store(vm_id, Ordering::Relaxed);
    }

    #[test]
    fn registered_vmexit_handler_observes_vm_id() {
        LAST_VMEXIT_VM_ID.store(0, Ordering::Relaxed);
        clear_vmexit_handler_for_test();

        register_vmexit_handler(record_vmexit);
        notify_vmexit(7);

        assert_eq!(LAST_VMEXIT_VM_ID.load(Ordering::Relaxed), 7);
        clear_vmexit_handler_for_test();
    }
}
