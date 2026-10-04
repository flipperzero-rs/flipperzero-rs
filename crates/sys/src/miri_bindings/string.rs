#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuriString {
    _unused: [u8; 0],
}
#[doc = "Allocate new FuriString.\n\n # Returns\n\npointer to the instance of FuriString"]
pub unsafe fn furi_string_alloc() -> *mut FuriString {
    todo!()
}
#[doc = "Allocate new FuriString and set it to string.\n\n Allocate & Set the string a to the string.\n\n # Arguments\n\n* `source` - The source FuriString instance\n\n # Returns\n\npointer to the new instance of FuriString"]
pub unsafe fn furi_string_alloc_set(source: *const FuriString) -> *mut FuriString {
    todo!()
}
#[doc = "Allocate new FuriString and set it to C string.\n\n Allocate & Set the string a to the C string.\n\n # Arguments\n\n* `cstr_source` - The C-string instance\n\n # Returns\n\npointer to the new instance of FuriString"]
pub unsafe fn furi_string_alloc_set_str(cstr_source: *const core::ffi::c_char) -> *mut FuriString {
    todo!()
}
#[doc = "Allocate new FuriString and move source string content to it.\n\n Allocate the string, set it to the other one, and destroy the other one.\n\n # Arguments\n\n* `source` - The source FuriString instance\n\n # Returns\n\npointer to the new instance of FuriString"]
pub unsafe fn furi_string_alloc_move(source: *mut FuriString) -> *mut FuriString {
    todo!()
}
#[doc = "Free FuriString.\n\n # Arguments\n\n* `string` - The FuriString instance to free"]
pub unsafe fn furi_string_free(string: *mut FuriString) {
    todo!()
}
#[doc = "Reserve memory for string.\n\n Modify the string capacity to be able to handle at least 'alloc' characters\n (including final null char).\n\n # Arguments\n\n* `string` - The FuriString instance\n * `size` - The size to reserve"]
pub unsafe fn furi_string_reserve(string: *mut FuriString, size: usize) {
    todo!()
}
#[doc = "Reset string.\n\n Make the string empty.\n\n # Arguments\n\n* `string` - The FuriString instance"]
pub unsafe fn furi_string_reset(string: *mut FuriString) {
    todo!()
}
#[doc = "Swap two strings.\n\n Swap the two strings string_1 and string_2.\n\n # Arguments\n\n* `string_1` - The FuriString instance 1\n * `string_2` - The FuriString instance 2"]
pub unsafe fn furi_string_swap(string_1: *mut FuriString, string_2: *mut FuriString) {
    todo!()
}
#[doc = "Move string_2 content to string_1.\n\n Copy data from one string to another and destroy the source.\n\n # Arguments\n\n* `destination` - The destination FuriString\n * `source` - The source FuriString"]
pub unsafe fn furi_string_move(destination: *mut FuriString, source: *mut FuriString) {
    todo!()
}
#[doc = "Compute a hash for the string.\n\n # Arguments\n\n* `string` - The FuriString instance\n\n # Returns\n\nhash value"]
pub unsafe fn furi_string_hash(string: *const FuriString) -> usize {
    todo!()
}
#[doc = "Get string size (usually length, but not for UTF-8)\n\n # Arguments\n\n* `string` - The FuriString instance\n\n # Returns\n\nsize of the string"]
pub unsafe fn furi_string_size(string: *const FuriString) -> usize {
    todo!()
}
#[doc = "Check that string is empty or not\n\n # Arguments\n\n* `string` - The FuriString instance\n\n # Returns\n\ntrue if empty otherwise false"]
pub unsafe fn furi_string_empty(string: *const FuriString) -> bool {
    todo!()
}
#[doc = "Get the character at the given index.\n\n Return the selected character of the string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `index` - The index\n\n # Returns\n\ncharacter at index"]
pub unsafe fn furi_string_get_char(string: *const FuriString, index: usize) -> core::ffi::c_char {
    todo!()
}
#[doc = "Return the string view a classic C string.\n\n # Arguments\n\n* `string` - The FuriString instance\n\n # Returns\n\nconst C-string, usable till first container change"]
pub unsafe fn furi_string_get_cstr(string: *const FuriString) -> *const core::ffi::c_char {
    todo!()
}
#[doc = "Set the string to the other string.\n\n Set the string to the source string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `source` - The source"]
pub unsafe fn furi_string_set(string: *mut FuriString, source: *mut FuriString) {
    todo!()
}
#[doc = "Set the string to the other C string.\n\n Set the string to the source C string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `source` - The source"]
pub unsafe fn furi_string_set_str(string: *mut FuriString, source: *const core::ffi::c_char) {
    todo!()
}
#[doc = "Set the string to the n first characters of the C string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `source` - The source\n * `length` - The length"]
pub unsafe fn furi_string_set_strn(
    string: *mut FuriString,
    source: *const core::ffi::c_char,
    length: usize,
) {
    todo!()
}
#[doc = "Set the character at the given index.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `index` - The index\n * `c` - The character"]
pub unsafe fn furi_string_set_char(string: *mut FuriString, index: usize, c: core::ffi::c_char) {
    todo!()
}
#[doc = "Set the string to the n first characters of other one.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `source` - The source\n * `offset` - The offset\n * `length` - The length"]
pub unsafe fn furi_string_set_n(
    string: *mut FuriString,
    source: *const FuriString,
    offset: usize,
    length: usize,
) {
    todo!()
}
#[doc = "Append a character to the string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `c` - The character"]
pub unsafe fn furi_string_push_back(string: *mut FuriString, c: core::ffi::c_char) {
    todo!()
}

#[doc = "An unicode value"]
pub type FuriStringUnicodeValue = core::ffi::c_uint;
#[doc = "Push unicode into string, encoding it in UTF8.\n\n # Arguments\n\n* `string` - The string\n * `unicode` - The unicode"]
pub unsafe fn furi_string_utf8_push(string: *mut FuriString, unicode: FuriStringUnicodeValue) {
    todo!()
}
#[doc = "Compare two strings and return the sort order.\n\n # Arguments\n\n* `string_1` - The string 1\n * `string_2` - The string 2\n\n # Returns\n\nzero if equal"]
pub unsafe fn furi_string_cmp(
    string_1: *const FuriString,
    string_2: *const FuriString,
) -> core::ffi::c_int {
    todo!()
}
#[doc = "Test if the string is equal to the C string.\n\n # Arguments\n\n* `string_1` - The string 1\n * `cstring_2` - The cstring 2\n\n # Returns\n\ntrue if equal false otherwise"]
pub unsafe fn furi_string_equal_str(
    string_1: *const FuriString,
    cstring_2: *const core::ffi::c_char,
) -> bool {
    todo!()
}
#[doc = "Append a string to the string.\n\n Concatenate the string with the other string.\n\n # Arguments\n\n* `string_1` - The string 1\n * `string_2` - The string 2"]
pub unsafe fn furi_string_cat(string_1: *mut FuriString, string_2: *const FuriString) {
    todo!()
}
#[doc = "Append a C string to the string.\n\n Concatenate the string with the C string.\n\n # Arguments\n\n* `string_1` - The string 1\n * `cstring_2` - The cstring 2"]
pub unsafe fn furi_string_cat_str(string_1: *mut FuriString, cstring_2: *const core::ffi::c_char) {
    todo!()
}
#[doc = "Search the first occurrence of the needle in the string from the position\n start.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `needle` - The needle\n * `start` - The start (By default, start is zero)\n\n # Returns\n\nposition or FURI_STRING_FAILURE if not found"]
pub unsafe fn furi_string_search_str(
    string: *const FuriString,
    needle: *const core::ffi::c_char,
    start: usize,
) -> usize {
    todo!()
}
#[doc = "Test if two strings are equal.\n\n # Arguments\n\n* `string_1` - The string 1\n * `string_2` - The string 2\n\n # Returns\n\ntrue if equal false otherwise"]
pub unsafe fn furi_string_equal(string_1: *const FuriString, string_2: *const FuriString) -> bool {
    todo!()
}
#[doc = "Trim the string left to the first 'index' bytes.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `index` - The index"]
pub unsafe fn furi_string_left(string: *mut FuriString, index: usize) {
    todo!()
}
#[doc = "Trim the string right from the 'index' position to the last position.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `index` - The index"]
pub unsafe fn furi_string_right(string: *mut FuriString, index: usize) {
    todo!()
}
#[doc = "Trim the string from position index to size bytes.\n\n See also furi_string_set_n.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `index` - The index\n * `size` - The size"]
pub unsafe fn furi_string_mid(string: *mut FuriString, index: usize, size: usize) {
    todo!()
}
#[doc = "Test if the string starts with the given string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `start` - The FuriString instance\n\n # Returns\n\ntrue if string starts with"]
pub unsafe fn furi_string_start_with(string: *const FuriString, start: *const FuriString) -> bool {
    todo!()
}
#[doc = "Test if the string starts with the given C string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `start` - The start\n\n # Returns\n\ntrue if string starts with"]
pub unsafe fn furi_string_start_with_str(
    string: *const FuriString,
    start: *const core::ffi::c_char,
) -> bool {
    todo!()
}
#[doc = "Test if the string ends with the given string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `end` - The end\n\n # Returns\n\ntrue if string ends with"]
pub unsafe fn furi_string_end_with(string: *const FuriString, end: *const FuriString) -> bool {
    todo!()
}
#[doc = "Test if the string ends with the given C string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `end` - The end\n\n # Returns\n\ntrue if string ends with"]
pub unsafe fn furi_string_end_with_str(
    string: *const FuriString,
    end: *const core::ffi::c_char,
) -> bool {
    todo!()
}
#[doc = "Search the first occurrence of the needle in the string from the position\n start.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `needle` - The needle\n * `start` - The start (By default, start is zero)\n\n # Returns\n\nposition or FURI_STRING_FAILURE if not found"]
pub unsafe fn furi_string_search(
    string: *const FuriString,
    needle: *const FuriString,
    start: usize,
) -> usize {
    todo!()
}
#[doc = "Search for the position of the character c from the position start (include)\n in the string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `c` - The character\n * `start` - The start (By default, start is zero)\n\n # Returns\n\nposition or FURI_STRING_FAILURE if not found"]
pub unsafe fn furi_string_search_char(
    string: *const FuriString,
    c: core::ffi::c_char,
    start: usize,
) -> usize {
    todo!()
}
#[doc = "Search for the position of the character c from the position start (include)\n in the string.\n\n # Arguments\n\n* `string` - The FuriString instance\n * `c` - The character\n * `start` - The start (By default, start is zero)\n\n # Returns\n\nposition or FURI_STRING_FAILURE if not found"]
pub unsafe fn furi_string_search_rchar(
    string: *const FuriString,
    c: core::ffi::c_char,
    start: usize,
) -> usize {
    todo!()
}
pub const FuriStringUTF8StateStarting: FuriStringUTF8State = FuriStringUTF8State(0);
pub const FuriStringUTF8StateDecoding1: FuriStringUTF8State = FuriStringUTF8State(1);
pub const FuriStringUTF8StateDecoding2: FuriStringUTF8State = FuriStringUTF8State(2);
pub const FuriStringUTF8StateDecoding3: FuriStringUTF8State = FuriStringUTF8State(3);
pub const FuriStringUTF8StateError: FuriStringUTF8State = FuriStringUTF8State(4);
#[repr(transparent)]
#[doc = "State of the UTF8 decoding machine state"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FuriStringUTF8State(pub core::ffi::c_uchar);
    #[doc = "Main generic UTF8 decoder\n\n It takes a character, and the previous state and the previous value of the\n unicode value. It updates the state and the decoded unicode value. A decoded\n unicode encoded value is valid only when the state is\n FuriStringUTF8StateStarting.\n\n # Arguments\n\n* `c` - The character\n * `state` - The state\n * `unicode` - The unicode"]
    pub unsafe fn furi_string_utf8_decode(
        c: core::ffi::c_char,
        state: *mut FuriStringUTF8State,
        unicode: *mut FuriStringUnicodeValue,
    ) { todo!() }
