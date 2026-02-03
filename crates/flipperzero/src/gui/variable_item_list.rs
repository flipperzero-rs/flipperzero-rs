//! Safe wrapper for dealing with allocations and freeing for the variable item list

use core::cell::UnsafeCell;
use core::marker::PhantomPinned;

use flipperzero_sys as sys;

#[repr(transparent)]
pub struct VariableItemList {
    raw: UnsafeCell<sys::VariableItemList>,
    _marker: PhantomPinned,
}

impl VariableItemList {
    /// Get Variable Item List reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::VariableItemList`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw<'a>(raw: *mut sys::VariableItemList) -> &'a Self {
        unsafe { &*(raw.cast::<VariableItemList>()) }
    }

    /// Get Variable Item List reference from raw pointer.
    ///
    /// # Safety
    /// Pointer must be non-null and point to a valid `sys::VariableItemList`.
    /// This pointer must outlive this reference.
    pub unsafe fn from_raw_mut<'a>(raw: *mut sys::VariableItemList) -> &'a mut Self {
        unsafe { &mut *(raw.cast::<VariableItemList>()) }
    }

    /// Get pointer to raw [`sys::VariableItemList`].
    pub fn as_ptr(&self) -> *mut sys::VariableItemList {
        self.raw.get()
    }
}

#[cfg(feature = "alloc")]
pub mod alloc {
    extern crate alloc;

    use crate::furi::string::FuriString;
    use alloc::{boxed::Box, vec::Vec};
    use core::{ffi::c_void, ptr};
    use flipperzero_sys as sys;

    pub struct VariableItemList<'a> {
        list: &'a mut super::VariableItemList,
        strings: Vec<FuriString>,
        items: Vec<ptr::NonNull<sys::VariableItem>>,
        on_click_callback: Option<Callback>,
    }

    /// what do you want to do when the OK button is clicked
    enum Callback {
        SameActionForAllInputs,
        UniqueCallbackForAllInputs(Vec<(usize, Box<dyn Fn() -> ()>)>),
    }

    impl VariableItemList<'_> {
        pub fn new() -> Self {
            Self {
                list: unsafe {
                    super::VariableItemList::from_raw_mut(sys::variable_item_list_alloc())
                },
                on_click_callback: None,
                strings: Vec::new(),
                items: Vec::new(),
            }
        }

        /// Get pointer to the underlying [`sys::VariableItemList`].
        pub fn as_ptr(&self) -> *mut sys::VariableItemList {
            self.list.as_ptr()
        }

        pub fn push_item_plaintext(&mut self, label: FuriString) -> () {
            let item = unsafe {
                sys::variable_item_list_add(
                    self.list.as_ptr(),
                    label.as_c_ptr(),
                    0,
                    None,
                    ptr::null_mut(),
                )
            };

            let item = ptr::NonNull::new(item)
                .expect("ptr returned from variable_item_list_add is never null");

            self.items.push(item);
            self.strings.push(label);
        }

        pub fn push_item_with_on_click_callback(
            &mut self,
            label: FuriString,
            callback: Box<dyn Fn() -> ()>,
        ) -> () {
            let item = unsafe {
                sys::variable_item_list_add(
                    self.list.as_ptr(),
                    label.as_c_ptr(),
                    0,
                    None,
                    ptr::null_mut(),
                )
            };

            let item: ptr::NonNull<flipperzero_sys::VariableItem> = ptr::NonNull::new(item)
                .expect("ptr returned from variable_item_list_add is never null");

            self.items.push(item);
            self.strings.push(label);

            if let Some(on_click_callback) = &mut self.on_click_callback {
                match on_click_callback {
                    Callback::UniqueCallbackForAllInputs(items) => {
                        items.push((self.items.len(), callback))
                    }
                    _ => todo!(),
                }
            } else {
                let mut items = Vec::new();
                items.push((self.items.len(), callback));

                unsafe {
                    sys::variable_item_list_set_enter_callback(
                        self.list.as_ptr(),
                        Some(unique_on_enter_callback),
                        ptr::from_mut(&mut items).cast::<c_void>(),
                    );
                };

                self.on_click_callback = Some(Callback::UniqueCallbackForAllInputs(items));
            }
        }

        pub fn clear(&mut self) -> () {
            unsafe { sys::variable_item_list_reset(self.list.as_ptr()) };
        }
    }

    unsafe extern "C" fn unique_on_enter_callback(context: *mut c_void, index: u32) -> () {
        todo!()
    }

    impl Drop for VariableItemList<'_> {
        fn drop(&mut self) {
            self.strings.clear();
            self.items.clear();

            unsafe { sys::variable_item_list_free(self.list.as_ptr()) };
        }
    }
}
