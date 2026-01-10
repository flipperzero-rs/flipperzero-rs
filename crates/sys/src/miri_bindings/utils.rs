unsafe extern "Rust" {
    pub(crate) fn miri_alloc(size: usize, align: usize) -> *mut u8;
    pub(crate) fn miri_dealloc(ptr: *mut u8, size: usize, align: usize);

    pub(crate) fn miri_thread_spawn(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;
    pub(crate) fn miri_thread_join(thread_id: usize) -> bool;

    pub(crate) safe fn miri_spin_loop();
}
