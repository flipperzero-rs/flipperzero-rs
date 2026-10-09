#[doc = "< Press event, emitted after debounce"]
pub const InputTypePress: InputType = InputType(0);
#[doc = "< Release event, emitted after debounce"]
pub const InputTypeRelease: InputType = InputType(1);
#[doc = "< Short event, emitted after InputTypeRelease done within INPUT_LONG_PRESS interval"]
pub const InputTypeShort: InputType = InputType(2);
#[doc = "< Long event, emitted after INPUT_LONG_PRESS_COUNTS interval, asynchronous to InputTypeRelease"]
pub const InputTypeLong: InputType = InputType(3);
#[doc = "< Repeat event, emitted with INPUT_LONG_PRESS_COUNTS period after InputTypeLong event"]
pub const InputTypeRepeat: InputType = InputType(4);
#[doc = "< Special value for exceptional"]
pub const InputTypeMAX: InputType = InputType(5);
#[repr(transparent)]
#[doc = "Input Types\n Some of them are physical events and some logical"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InputType(pub core::ffi::c_uchar);
#[doc = "Input Event, dispatches with FuriPubSub"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputEvent {
    pub __bindgen_anon_1: InputEvent__bindgen_ty_1,
    pub key: InputKey,
    pub type_: InputType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union InputEvent__bindgen_ty_1 {
    pub sequence: u32,
    pub __bindgen_anon_1: InputEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputEvent__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: super::__BindgenBitfieldUnit<[u8; 4usize]>,
}
impl InputEvent__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub unsafe fn sequence_source(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub unsafe fn set_sequence_source(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn sequence_source_raw(this: *const Self) -> u8 {
        unsafe {
            ::core::mem::transmute(<super::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                2u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_sequence_source_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            <super::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn sequence_counter(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 30u8) as u32) }
    }
    #[inline]
    pub unsafe fn set_sequence_counter(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn sequence_counter_raw(this: *const Self) -> u32 {
        unsafe {
            ::core::mem::transmute(<super::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                2usize,
                30u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_sequence_counter_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            <super::__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                2usize,
                30u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub unsafe fn new_bitfield_1(
        sequence_source: u8,
        sequence_counter: u32,
    ) -> super::__BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: super::__BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let sequence_source: u8 = unsafe { ::core::mem::transmute(sequence_source) };
            sequence_source as u64
        });
        __bindgen_bitfield_unit.set(2usize, 30u8, {
            let sequence_counter: u32 = unsafe { ::core::mem::transmute(sequence_counter) };
            sequence_counter as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const InputKeyUp: InputKey = InputKey(0);
pub const InputKeyDown: InputKey = InputKey(1);
pub const InputKeyRight: InputKey = InputKey(2);
pub const InputKeyLeft: InputKey = InputKey(3);
pub const InputKeyOk: InputKey = InputKey(4);
pub const InputKeyBack: InputKey = InputKey(5);
#[doc = "< Special value"]
pub const InputKeyMAX: InputKey = InputKey(6);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct InputKey(pub core::ffi::c_uchar);

#[doc = "Get human readable input key name\n # Arguments\n\n* `key` - - InputKey\n # Returns\n\nstring"]
pub unsafe fn input_get_key_name(key: InputKey) -> *const core::ffi::c_char {
    match key {
        InputKeyUp => c"Up",
        InputKeyDown => c"Down",
        InputKeyRight => c"Right",
        InputKeyLeft => c"Left",
        InputKeyOk => c"Ok",
        InputKeyBack => c"Back",
        _ => c"Err",
    }.as_ptr()
}

pub unsafe fn input_get_type_name(type_: InputType) -> *const core::ffi::c_char {
    match type_ {
        InputTypePress => c"Press",
        InputTypeRelease => c"Release",
        InputTypeShort => c"Short",
        InputTypeLong => c"Long",
        InputTypeRepeat => c"Repeat",
        _ => c"Err",
    }.as_ptr()
}
