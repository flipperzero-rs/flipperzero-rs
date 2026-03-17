#![allow(unused_variables)]

mod event;
mod gui;
mod input;
mod kernel;
mod mutex;
mod string;
mod thread;
mod utils;
mod version;

pub extern crate alloc;

pub use event::*;
pub use gui::*;
pub use input::*;
pub use kernel::*;
pub use mutex::*;
pub use string::*;
pub use thread::*;
pub use version::*;

use alloc::sync::Arc;
use core::cell::OnceCell;
use core::ffi::{CStr, c_void};

pub const API_VERSION: u32 = 5701633;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize)
        };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
#[doc = "< Wait for any flag (default)."]
pub const FuriFlagWaitAny: FuriFlag = FuriFlag(0);
#[doc = "< Wait for all flags."]
pub const FuriFlagWaitAll: FuriFlag = FuriFlag(1);
#[doc = "< Do not clear flags which have been specified to wait for."]
pub const FuriFlagNoClear: FuriFlag = FuriFlag(2);
#[doc = "< Error indicator."]
pub const FuriFlagError: FuriFlag = FuriFlag(2147483648);
#[doc = "< FuriStatusError (-1)."]
pub const FuriFlagErrorUnknown: FuriFlag = FuriFlag(4294967295);
#[doc = "< FuriStatusErrorTimeout (-2)."]
pub const FuriFlagErrorTimeout: FuriFlag = FuriFlag(4294967294);
#[doc = "< FuriStatusErrorResource (-3)."]
pub const FuriFlagErrorResource: FuriFlag = FuriFlag(4294967293);
#[doc = "< FuriStatusErrorParameter (-4)."]
pub const FuriFlagErrorParameter: FuriFlag = FuriFlag(4294967292);
#[doc = "< FuriStatusErrorISR (-6)."]
pub const FuriFlagErrorISR: FuriFlag = FuriFlag(4294967290);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriFlag(pub core::ffi::c_uint);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DateTime {
    #[doc = "< Hour in 24H format: 0-23"]
    pub hour: u8,
    #[doc = "< Minute: 0-59"]
    pub minute: u8,
    #[doc = "< Second: 0-59"]
    pub second: u8,
    #[doc = "< Current day: 1-31"]
    pub day: u8,
    #[doc = "< Current month: 1-12"]
    pub month: u8,
    #[doc = "< Current year: 2000-2099"]
    pub year: u16,
    #[doc = "< Current weekday: 1-7"]
    pub weekday: u8,
}

#[doc = "< Operation completed successfully."]
pub const FuriStatusOk: FuriStatus = FuriStatus(0);
pub const FuriStatusError: FuriStatus = FuriStatus(-1);
#[doc = "< Operation not completed within the timeout period."]
pub const FuriStatusErrorTimeout: FuriStatus = FuriStatus(-2);
#[doc = "< Resource not available."]
pub const FuriStatusErrorResource: FuriStatus = FuriStatus(-3);
#[doc = "< Parameter error."]
pub const FuriStatusErrorParameter: FuriStatus = FuriStatus(-4);
pub const FuriStatusErrorNoMemory: FuriStatus = FuriStatus(-5);
pub const FuriStatusErrorISR: FuriStatus = FuriStatus(-6);
#[doc = "< Prevents enum down-size compiler optimization."]
pub const FuriStatusReserved: FuriStatus = FuriStatus(2147483647);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriStatus(pub core::ffi::c_int);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CliCommandFlag(pub core::ffi::c_uchar);

#[doc = "< Read access"]
pub const FSAM_READ: FS_AccessMode = FS_AccessMode(1);
#[doc = "< Write access"]
pub const FSAM_WRITE: FS_AccessMode = FS_AccessMode(2);
#[doc = "< Read and write access"]
pub const FSAM_READ_WRITE: FS_AccessMode = FS_AccessMode(3);
#[repr(transparent)]
#[doc = "Access mode flags"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FS_AccessMode(pub core::ffi::c_uchar);

#[repr(transparent)]
#[doc = "FileInfo flags"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FS_Flags(pub core::ffi::c_uchar);

#[doc = "< Open file, fail if file doesn't exist"]
pub const FSOM_OPEN_EXISTING: FS_OpenMode = FS_OpenMode(1);
#[doc = "< Open file. Create new file if not exist"]
pub const FSOM_OPEN_ALWAYS: FS_OpenMode = FS_OpenMode(2);
#[doc = "< Open file. Create new file if not exist. Set R/W pointer to EOF"]
pub const FSOM_OPEN_APPEND: FS_OpenMode = FS_OpenMode(4);
#[doc = "< Creates a new file. Fails if the file is exist"]
pub const FSOM_CREATE_NEW: FS_OpenMode = FS_OpenMode(8);
#[doc = "< Creates a new file. If file exist, truncate to zero size"]
pub const FSOM_CREATE_ALWAYS: FS_OpenMode = FS_OpenMode(16);
#[repr(transparent)]
#[doc = "Open mode flags"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FS_OpenMode(pub core::ffi::c_uchar);

#[repr(transparent)]
#[doc = "Enumeration of possible NFC HAL events."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriHalNfcEvent(pub core::ffi::c_ushort);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriHalRtcFlag(pub core::ffi::c_uchar);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct iButtonProtocolFeature(pub core::ffi::c_uchar);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Light(pub core::ffi::c_uchar);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MfUltralightFeatureSupport(pub core::ffi::c_ushort);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SubGhzProtocolFlag(pub core::ffi::c_ushort);

#[doc = "Gpio structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpioPin {
    pub port: *mut GPIO_TypeDef,
    pub pin: u16,
}

#[doc = "General Purpose I/O"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GPIO_TypeDef {
    #[doc = "< GPIO port mode register, Address offset: 0x00"]
    pub MODER: u32,
    #[doc = "< GPIO port output type register, Address offset: 0x04"]
    pub OTYPER: u32,
    #[doc = "< GPIO port output speed register, Address offset: 0x08"]
    pub OSPEEDR: u32,
    #[doc = "< GPIO port pull-up/pull-down register, Address offset: 0x0C"]
    pub PUPDR: u32,
    #[doc = "< GPIO port input data register, Address offset: 0x10"]
    pub IDR: u32,
    #[doc = "< GPIO port output data register, Address offset: 0x14"]
    pub ODR: u32,
    #[doc = "< GPIO port bit set/reset register, Address offset: 0x18"]
    pub BSRR: u32,
    #[doc = "< GPIO port configuration lock register, Address offset: 0x1C"]
    pub LCKR: u32,
    #[doc = "< GPIO alternate function registers, Address offset: 0x20-0x24"]
    pub AFR: [u32; 2usize],
    #[doc = "< GPIO Bit Reset register, Address offset: 0x28"]
    pub BRR: u32,
}

static GUI: lock::SpinLock<OnceCell<Arc<Gui>>> = lock::SpinLock::new(OnceCell::new(), b"GUI");

#[doc = "Open record\n\n # Arguments\n\n* `name` - record name\n\n # Returns\n\npointer to the record\n > **Note:** Thread safe. Open and close must be executed from the same\n thread. Suspends caller thread till record is available"]
pub unsafe fn furi_record_open(name: *const core::ffi::c_char) -> *mut c_void {
    let name = unsafe { CStr::from_ptr(name) };
    if name == c"gui" {
        let gui_cell = GUI.lock(b"record acquire");
        match gui_cell.get() {
            Some(_gui) => {
                todo!("we currently don't support the same record being opened multiple times")
            }
            None => {
                let gui: Arc<Gui> = GuiInner::spawn();

                // Gui is owned by the background Gui service thread, and also by this thread
                debug_assert_eq!(
                    Arc::strong_count(&gui),
                    2,
                    "[furi_record_open, gui service thread]"
                );
                let _ = gui_cell.set(gui.clone());
                debug_assert_eq!(
                    Arc::strong_count(&gui),
                    3,
                    "[furi_record_open, static cell, gui service thread]"
                );
                let gui_ptr: *const Gui = Arc::into_raw(gui.clone());
                debug_assert_eq!(
                    Arc::strong_count(&gui),
                    4,
                    "[furi_record open (local), furi_record open (to return), static cell, gui service thread]"
                );
                gui_ptr.cast::<c_void>().cast_mut()
            }
        }
    } else {
        unimplemented!()
    }
}

#[doc = "Close record\n\n # Arguments\n\n* `name` - record name\n > **Note:** Thread safe. Open and close must be executed from the same\n thread."]
pub unsafe fn furi_record_close(name: *const core::ffi::c_char) {
    let name = unsafe { CStr::from_ptr(name) };
    if name == c"gui" {
        let mut gui_cell = GUI.lock(b"record close");
        /*{
            let gui = gui_cell.get().unwrap();
            assert_eq!(
                Arc::strong_count(&gui),
                3,
                "[unsafe record (needs manually dropping), gui service thread, static cell]"
            );
        }*/
        let gui: Arc<lock::SpinLock<GuiInner>> =
            OnceCell::take(&mut gui_cell).expect("GUI must have been opened before being closed");
        // This method is called on UnsafeRecord, which owns a copy of the Arc<Gui>. As such, there
        // should only be three references at this point;
        // 1. in the static, that we just took,
        // 2. one in the UnsafeRecord.data
        // 3. one held by the Gui service thread
        /*assert_eq!(
            Arc::strong_count(&gui),
            3,
            "[unsafe record (needs manually dropping), gui service thread, local from static cell]"
        );*/

        let gui_thread_id = {
            let mut gui = gui.lock(b"record close");
            gui.stop = true;
            gui.thread_id
        };

        unsafe { utils::miri_thread_join(gui_thread_id) };

        /*
        assert_eq!(
            Arc::strong_count(&gui),
            2,
            "[unsafe record (needs manually dropping), local]"
        );*/
        // We drop Gui here, and then the only remaining reference to the Arc is in the Record,
        // which will go out of scope immediate after this when the record is dropped
        unsafe { Arc::decrement_strong_count(Arc::as_ptr(&gui)) };
    } else {
        unimplemented!()
    }
}

#[doc = "Get current tick counter\n\n System uptime, may overflow.\n\n # Returns\n\nCurrent ticks in milliseconds"]
pub unsafe fn furi_get_tick() -> u32 {
    todo!()
}
#[doc = "Delay execution\n\n This should never be called in interrupt request context.\n\n Also keep in mind delay is aliased to scheduler timer intervals.\n\n # Arguments\n\n* `ticks` (direction in) - The ticks count to pause"]
pub unsafe fn furi_delay_tick(ticks: u32) {
    // NOTE: none of the tests we're writing care about specific timing, so we're just spinning
    // here to allow for another thread to take over
    utils::miri_spin_loop();
}
#[doc = "Delay in milliseconds\n\n This method uses kernel ticks on the inside, which causes delay to be aliased to scheduler timer intervals.\n Real wait time will be between X+ milliseconds.\n Special value: 0, will cause task yield.\n Also if used when kernel is not running will fall back to `furi_delay_us`.\n\n Cannot be used from ISR\n\n # Arguments\n\n* `milliseconds` (direction in) - milliseconds to wait"]
pub unsafe fn furi_delay_ms(milliseconds: u32) {
    // NOTE: none of the tests we're writing care about specific timing, so we're just spinning
    // here to allow for another thread to take over
    utils::miri_spin_loop();
}
#[doc = "Delay in microseconds\n\n Implemented using Cortex DWT counter. Blocking and non aliased.\n\n # Arguments\n\n* `microseconds` (direction in) - microseconds to wait"]
pub unsafe fn furi_delay_us(microseconds: u32) {
    // NOTE: none of the tests we're writing care about specific timing, so we're just spinning
    // here to allow for another thread to take over
    utils::miri_spin_loop();
}

pub const FuriLogLevelDefault: FuriLogLevel = FuriLogLevel(0);
pub const FuriLogLevelNone: FuriLogLevel = FuriLogLevel(1);
pub const FuriLogLevelError: FuriLogLevel = FuriLogLevel(2);
pub const FuriLogLevelWarn: FuriLogLevel = FuriLogLevel(3);
pub const FuriLogLevelInfo: FuriLogLevel = FuriLogLevel(4);
pub const FuriLogLevelDebug: FuriLogLevel = FuriLogLevel(5);
pub const FuriLogLevelTrace: FuriLogLevel = FuriLogLevel(6);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriLogLevel(pub core::ffi::c_uchar);

#[doc = "Get log level\n\n # Returns\n\nThe furi log level."]
pub unsafe fn furi_log_get_level() -> FuriLogLevel {
    todo!()
}
#[doc = "Print log record\n\n # Arguments\n\n* `level` -\n * `tag` -\n * `format` -\n * `...` -"]
pub unsafe fn furi_log_print_format(
    level: FuriLogLevel,
    tag: *const core::ffi::c_char,
    format: *const core::ffi::c_char,
) {
    todo!()
}

#[doc = "Get free heap size\n\n # Returns\n\nfree heap size in bytes"]
pub unsafe fn memmgr_get_free_heap() -> usize {
    todo!()
}
#[doc = "Get total heap size\n\n # Returns\n\ntotal heap size in bytes"]
pub unsafe fn memmgr_get_total_heap() -> usize {
    todo!()
}
#[doc = "Get heap watermark\n\n # Returns\n\nminimum heap in bytes"]
pub unsafe fn memmgr_get_minimum_free_heap() -> usize {
    todo!()
}

pub(super) mod lock {
    use crate::miri_bindings::utils::miri_spin_loop;
    // use crate::miri_bindings::utils::miri_write_to_stdout;
    use core::cell::UnsafeCell;
    use core::ops::{Deref, DerefMut};
    use core::sync::atomic::{AtomicBool, Ordering};

    fn miri_write_to_stdout(bytes: &[u8]) {}

    pub struct SpinLock<T> {
        data: UnsafeCell<T>,
        inner: AtomicBool,
        name: &'static [u8],
    }

    pub struct SpinLockGuard<'a, T> {
        lock: &'a SpinLock<T>,
        to: &'a [u8],
    }

    unsafe impl<T> Sync for SpinLock<T> {}

    impl<T> SpinLock<T> {
        pub const fn new(data: T, name: &'static [u8]) -> Self {
            Self {
                data: UnsafeCell::new(data),
                inner: AtomicBool::new(false),
                name,
            }
        }

        pub fn lock<'a>(&'a self, to: &'a [u8]) -> SpinLockGuard<'a, T> {
            miri_write_to_stdout(b"\tAttempting to lock ");
            miri_write_to_stdout(self.name);
            miri_write_to_stdout(b" for ");
            miri_write_to_stdout(to);
            miri_write_to_stdout(b"\n");
            // NOTE: SeqCst has been used all over here, bcs it's definitely correct, and I haven't got
            // a good enough handle on the other orderings to pick one that would also be correct but
            // more efficient.
            while !self
                .inner
                .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                miri_spin_loop();
            }

            miri_write_to_stdout(b"\tAcquired lock around ");
            miri_write_to_stdout(self.name);
            miri_write_to_stdout(b" for ");
            miri_write_to_stdout(to);
            miri_write_to_stdout(b"\n");
            SpinLockGuard { lock: self, to }
        }

        pub unsafe fn deref_unsafe(&self) -> &T {
            unsafe { &*self.data.get() }
        }
    }

    impl<'a, T> Deref for SpinLockGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { &mut *self.lock.data.get() }
        }
    }

    impl<T> SpinLockGuard<'_, T> {
        pub(crate) fn unlock(&mut self) {
            miri_write_to_stdout(b"\tUnlock ");
            miri_write_to_stdout(self.lock.name);
            miri_write_to_stdout(b", held by ");
            miri_write_to_stdout(self.to);
            miri_write_to_stdout(b"\n");
            // NOTE: SeqCst has been used all over here, bcs it's definitely correct, and I haven't got
            // a good enough handle on the other orderings to pick one that would also be correct but
            // more efficient.
            self.lock.inner.store(false, Ordering::SeqCst);
        }

        pub(crate) fn reacquire(&mut self) {
            while !self
                .lock
                .inner
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                miri_spin_loop();
            }
        }
    }

    impl<'a, T> Drop for SpinLockGuard<'a, T> {
        fn drop(&mut self) {
            self.unlock();
        }
    }
}

pub struct CallbackWithContext<T> {
    pub callback: T,
    pub context: *mut c_void,
}
