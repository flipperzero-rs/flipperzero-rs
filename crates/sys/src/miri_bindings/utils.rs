unsafe extern "Rust" {
    pub fn miri_alloc(size: usize, align: usize) -> *mut u8;
    pub fn miri_dealloc(ptr: *mut u8, size: usize, align: usize);

    pub fn miri_thread_spawn(t: extern "Rust" fn(*mut ()), data: *mut ()) -> usize;
    pub fn miri_thread_join(thread_id: usize) -> bool;

    pub safe fn miri_spin_loop();

    pub safe fn miri_write_to_stdout(bytes: &[u8]);
}
