//! String primitives built around `FuriString`.

use core::{
    borrow::Borrow,
    cmp::Ordering,
    convert::Infallible,
    ffi::{c_char, CStr},
    fmt::{self, Write},
    hash,
    mem::ManuallyDrop,
    ops::{Add, AddAssign},
    ptr::{self, NonNull},
};

#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, boxed::Box, ffi::CString};

use flipperzero_sys as sys;

mod iter;
use self::iter::{Bytes, CharIndices, Chars};

mod pattern;
use self::pattern::Pattern;

/// Source: [UnicodeSet `[:White_Space=Yes:]`][src].
///
/// [src]: https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=[%3AWhite_Space%3DYes%3A]
const WHITESPACE: &[char] = &[
    ' ', '\x09', '\x0a', '\x0b', '\x0c', '\x0d', '\u{0085}', '\u{00A0}', '\u{1680}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}', '\u{205F}', '\u{3000}',
];

/// A Furi string.
///
/// This is similar to Rust's [`CString`] in that it represents an owned, C-compatible,
/// nul-terminated string with no nul bytes in the middle. It also has additional methods
/// to provide the flexibility of Rust's [`String`].
/// It is used in various APIs of the Flipper Zero SDK.
///
/// This type does not require the `alloc` feature flag, because it does not use the Rust
/// allocator. Very short strings (7 bytes or fewer) are stored directly inside the
/// `FuriString` struct (which is stored on the heap), while longer strings are allocated
/// on the heap by the Flipper Zero firmware.
///
/// [`CString`]: https://doc.rust-lang.org/nightly/alloc/ffi/struct.CString.html
/// [`String`]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html
#[derive(Eq)]
pub struct FuriString(NonNull<sys::FuriString>);

impl Drop for FuriString {
    fn drop(&mut self) {
        unsafe { sys::furi_string_free(self.0.as_ptr()) };
    }
}

// Implementations matching `std::string::String`.
impl FuriString {
    /// Creates a new empty `FuriString`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        FuriString(unsafe { NonNull::new_unchecked(sys::furi_string_alloc()) })
    }

    /// Creates a new empty `FuriString` with at least the specified capacity.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let s = Self::new();
        // The size passed to `sys::furi_string_reserve` needs to include the nul
        // terminator.
        unsafe { sys::furi_string_reserve(s.0.as_ptr(), capacity + 1) };
        s
    }

    /// Consume the [`FuriString`] and return the internal [`sys::FuriString`].
    /// You are responsible for freeing the returned [`sys::FuriString`] using
    /// [`sys::furi_string_free`] or similar API.
    #[inline]
    #[must_use]
    pub fn into_raw(self) -> NonNull<sys::FuriString> {
        // Inhibit calling of the `Drop` trait
        let s = ManuallyDrop::new(self);

        s.0
    }

    /// Extracts a pointer to a raw zero-terminated string
    /// containing the entire string slice.
    #[inline]
    #[must_use]
    pub fn as_c_ptr(&self) -> *const c_char {
        let ptr = self.0.as_ptr();
        // SAFETY: raw pointer is valid
        unsafe { sys::furi_string_get_cstr(ptr) }
    }

    /// Extracts a `CStr` containing the entire string slice, with nul termination.
    #[inline]
    #[must_use]
    pub fn as_c_str(&self) -> &CStr {
        let c_ptr = self.as_c_ptr();
        // SAFETY: `c_ptr` has just been extracted from a valid `FuriString`
        unsafe { CStr::from_ptr(c_ptr) }
    }

    /// Raw pointer to the inner [`sys::FuriString`].
    /// You must not deallocate, free or otherwise invalidate this pointer otherwise undefined behaviour will result.
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut sys::FuriString {
        self.0.as_ptr()
    }

    /// Appends a given `FuriString` onto the end of this `FuriString`.
    #[inline]
    pub fn push_string(&mut self, string: &FuriString) {
        unsafe { sys::furi_string_cat(self.0.as_ptr(), string.0.as_ptr()) }
    }

    /// Appends a given `str` onto the end of this `FuriString`.
    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.extend(string.chars());
    }

    /// Appends a given `CStr` onto the end of this `FuriString`.
    #[inline]
    pub fn push_c_str(&mut self, string: &CStr) {
        unsafe { sys::furi_string_cat_str(self.0.as_ptr(), string.as_ptr()) }
    }

    /// Reserves capacity for at least `additional` bytes more than the current length.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        // `self.len()` counts the number of bytes excluding the terminating nul, but the
        // size passed to `sys::furi_string_reserve` needs to include the nul terminator.
        unsafe { sys::furi_string_reserve(self.0.as_ptr(), self.len() + additional + 1) };
    }

    /// Appends the given [`char`] to the end of this `FuriString`.
    #[inline]
    pub fn push(&mut self, ch: char) {
        match ch.len_utf8() {
            1 => unsafe { sys::furi_string_push_back(self.0.as_ptr(), ch as c_char) },
            _ => unsafe {
                sys::furi_string_utf8_push(self.0.as_ptr(), ch as sys::FuriStringUnicodeValue)
            },
        }
    }

    /// Returns a byte slice of this `FuriString`'s contents.
    ///
    /// The returned slice will **not** contain the trailing nul terminator that the
    /// underlying C string has.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    /// Returns a byte slice of this `FuriString`'s contents with the trailing nul byte.
    ///
    /// This function is the equivalent of [`FuriString::to_bytes`] except that it will
    /// retain the trailing nul terminator instead of chopping it off.
    #[inline]
    #[must_use]
    pub fn to_bytes_with_nul(&self) -> &[u8] {
        self.as_c_str().to_bytes_with_nul()
    }

    /// Shortens this `FuriString` to the specified length.
    ///
    /// If `new_len` is greater than the string's current length, this has no effect.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        unsafe { sys::furi_string_left(self.0.as_ptr(), new_len) };
    }

    /// Inserts a character into this `FuriString` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `FuriString`'s length.
    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) {
        let mut buf = [0; 4];
        let encoded = ch.encode_utf8(&mut buf).as_bytes();
        self.insert_bytes(idx, encoded);
    }

    /// Inserts a string slice into this `FuriString` at a byte position.
    ///
    /// This is an *O*(*n*) operation as it requires copying every element in the buffer.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is larger than the `FuriString`'s length.
    #[inline]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.insert_bytes(idx, string.as_bytes());
    }

    fn insert_bytes(&mut self, idx: usize, bytes: &[u8]) {
        let len = self.len();
        assert!(idx <= len);

        // Reserve sufficient space to insert `bytes` without repeat re-allocations.
        let amt = bytes.len();
        self.reserve(amt);

        // Append `bytes` to force the underlying `FuriString` to be the correct length.
        for byte in bytes.iter() {
            unsafe { sys::furi_string_push_back(self.0.as_ptr(), *byte as c_char) };
        }

        // Now use pointer access to construct the expected `FuriString` contents.
        let c_ptr = self.as_c_ptr().cast::<u8>();
        unsafe {
            ptr::copy(c_ptr.add(idx), c_ptr.cast_mut().add(idx + amt), len - idx);
            ptr::copy_nonoverlapping(bytes.as_ptr(), c_ptr.cast_mut().add(idx), amt);
        }
    }

    /// Returns the length of this `FuriString`.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words, it might not
    /// be what a human considers the length of the string.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { sys::furi_string_size(self.0.as_ptr()) }
    }

    /// Returns `true` if this `FuriString` has a length of zero, and `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        unsafe { sys::furi_string_empty(self.0.as_ptr()) }
    }

    /// Splits the string into two at the given byte index.
    ///
    /// Returns a newly allocated `String`. `self` contains bytes `[0, at)`, and the
    /// returned `String` contains bytes `[at, len)`.
    ///
    /// Note that the capacity of `self` does not change.
    ///
    /// # Panics
    ///
    /// Panics if `at` is beyond the last byte of the string.
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> FuriString {
        // SAFETY: Trimming the beginning of a C string results in a valid C string, as
        // long as the nul byte is not trimmed.
        assert!(at <= self.len());
        let ret = FuriString(unsafe {
            NonNull::new_unchecked(sys::furi_string_alloc_set_str(self.as_c_ptr().add(at)))
        });
        self.truncate(at);
        ret
    }

    /// Truncates this `FuriString`, removing all contents.
    ///
    /// While this means the `FuriString` will have a length of zero, it does not touch
    /// its capacity.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { sys::furi_string_reset(self.0.as_ptr()) };
    }
}

// Implementations matching `str` that we don't already have from `std::string::String`
// and that are useful for a non-slice string. Some of these are altered to be mutating
// as we can't provide string slices.
impl FuriString {
    /// Returns an iterator over the [`char`]s of a `FuriString`.
    ///
    /// A `FuriString` might not contain valid UTF-8 (for example, if it represents a
    /// string obtained through the Flipper Zero SDK).Any invalid UTF-8 sequences will be
    /// replaced with [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �
    ///
    /// [U+FFFD]: core::char::REPLACEMENT_CHARACTER
    ///
    /// It's important to remember that [`char`] represents a Unicode Scalar
    /// Value, and might not match your idea of what a 'character' is. Iteration
    /// over grapheme clusters may be what you actually want.
    #[inline]
    pub fn chars_lossy(&self) -> Chars<'_> {
        Chars {
            iter: self.to_bytes().iter(),
        }
    }

    /// Returns an iterator over the [`char`]s of a `FuriString`, and their positions.
    ///
    /// A `FuriString` might not contain valid UTF-8 (for example, if it represents a
    /// string obtained through the Flipper Zero SDK).Any invalid UTF-8 sequences will be
    /// replaced with [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �
    ///
    /// [U+FFFD]: core::char::REPLACEMENT_CHARACTER
    ///
    /// The iterator yields tuples. The position is first, the [`char`] is second.
    #[inline]
    pub fn char_indices_lossy(&self) -> CharIndices<'_> {
        CharIndices {
            front_offset: 0,
            iter: self.chars_lossy(),
        }
    }

    /// An iterator over the bytes of a string slice.
    ///
    /// As a string consists of a sequence of bytes, we can iterate through a string by
    /// byte. This method returns such an iterator.
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.to_bytes().iter().copied())
    }

    /// Returns `true` if the given pattern `pat` matches a sub-slice of this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    #[inline]
    pub fn contains<P: Pattern>(&self, pat: P) -> bool {
        pat.is_contained_in(self)
    }

    /// Returns `true` if the given pattern `pat` matches a prefix of this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    pub fn starts_with<P: Pattern>(&self, pat: P) -> bool {
        pat.is_prefix_of(self)
    }

    /// Returns `true` if the given pattern `pat` matches a suffix of this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    pub fn ends_with<P: Pattern>(&self, pat: P) -> bool {
        pat.is_suffix_of(self)
    }

    /// Returns the byte index of the first byte of this string that matches the pattern `pat`.
    ///
    /// Returns [`None`] if the pattern doesn't match.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    #[inline]
    pub fn find<P: Pattern>(&self, pat: P) -> Option<usize> {
        pat.find_in(self)
    }

    /// Returns the byte index for the first byte of the last match of the pattern `pat`
    /// in this string.
    ///
    /// Returns [`None`] if the pattern doesn't match.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    #[inline]
    pub fn rfind<P: Pattern>(&self, pat: P) -> Option<usize> {
        pat.rfind_in(self)
    }

    /// Removes leading and trailing whitespace from this string.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core
    /// Property `White_Space`, which includes newlines.
    #[inline]
    pub fn trim(&mut self) {
        self.trim_matches(WHITESPACE)
    }

    /// Removes leading whitespace from this string.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core
    /// Property `White_Space`, which includes newlines.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first position
    /// of that byte string; for a left-to-right language like English or Russian, this
    /// will be left side, and for right-to-left languages like Arabic or Hebrew, this
    /// will be the right side.
    #[inline]
    pub fn trim_start(&mut self) {
        self.trim_start_matches(WHITESPACE)
    }

    /// Removes trailing whitespace from this string.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core
    /// Property `White_Space`, which includes newlines.
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `end` in this context means the last position of
    /// that byte string; for a left-to-right language like English or Russian, this will
    /// be right side, and for right-to-left languages like Arabic or Hebrew, this will be
    /// the left side.
    #[inline]
    pub fn trim_end(&mut self) {
        self.trim_end_matches(WHITESPACE)
    }

    /// Repeatedly removes from this string all prefixes and suffixes that match a pattern.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    pub fn trim_matches<P: Pattern + Copy>(&mut self, pat: P) {
        self.trim_start_matches(pat);
        self.trim_end_matches(pat);
    }

    /// Repeatedly removes from this string all prefixes that match a pattern `pat`.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `start` in this context means the first position
    /// of that byte string; for a left-to-right language like English or Russian, this
    /// will be left side, and for right-to-left languages like Arabic or Hebrew, this
    /// will be the right side.
    pub fn trim_start_matches<P: Pattern + Copy>(&mut self, pat: P) {
        while self.strip_prefix(pat) {}
    }

    /// Repeatedly removes from this string all suffixes that match a pattern `pat`.
    ///
    /// The pattern can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    ///
    /// # Text directionality
    ///
    /// A string is a sequence of bytes. `end` in this context means the last position of
    /// that byte string; for a left-to-right language like English or Russian, this will
    /// be right side, and for right-to-left languages like Arabic or Hebrew, this will be
    /// the left side.
    pub fn trim_end_matches<P: Pattern + Copy>(&mut self, pat: P) {
        while self.strip_suffix(pat) {}
    }

    /// Removes the given `prefix` from this string.
    ///
    /// If the string starts with the pattern `prefix`, returns `true`. Unlike
    /// [`Self::trim_start_matches`], this method removes the prefix exactly once.
    ///
    /// If the string does not start with `prefix`, returns `false`.
    ///
    /// The prefix can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    #[must_use]
    pub fn strip_prefix<P: Pattern>(&mut self, prefix: P) -> bool {
        prefix.strip_prefix_of(self)
    }

    /// Removes the given `suffix` from this string.
    ///
    /// If the string ends with the pattern `suffix`, returns `true`. Unlike
    /// [`Self::trim_end_matches`], this method removes the suffix exactly once.
    ///
    /// If the string does not end with `suffix`, returns `false`.
    ///
    /// The suffix can be a `&FuriString`, [`c_char`], `&CStr`, [`char`], or a slice of
    /// [`char`]s.
    ///
    /// [`char`]: prim@char
    #[must_use]
    pub fn strip_suffix<P: Pattern>(&mut self, suffix: P) -> bool {
        suffix.strip_suffix_of(self)
    }
}

impl Clone for FuriString {
    fn clone(&self) -> Self {
        Self(unsafe { NonNull::new_unchecked(sys::furi_string_alloc_set(self.0.as_ptr())) })
    }
}

impl Default for FuriString {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<CStr> for FuriString {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Borrow<CStr> for FuriString {
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

impl From<char> for FuriString {
    fn from(value: char) -> Self {
        let mut buf = FuriString::with_capacity(value.len_utf8());
        buf.push(value);
        buf
    }
}

impl From<&str> for FuriString {
    fn from(value: &str) -> Self {
        let mut buf = FuriString::with_capacity(value.len());
        buf.push_str(value);
        buf
    }
}

impl From<&mut str> for FuriString {
    fn from(value: &mut str) -> Self {
        From::<&str>::from(value)
    }
}

impl From<&CStr> for FuriString {
    fn from(value: &CStr) -> Self {
        Self(unsafe { NonNull::new_unchecked(sys::furi_string_alloc_set_str(value.as_ptr())) })
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl From<Box<str>> for FuriString {
    fn from(value: Box<str>) -> Self {
        FuriString::from(value.as_ref())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a> From<Cow<'a, str>> for FuriString {
    fn from(value: Cow<'a, str>) -> Self {
        FuriString::from(value.as_ref())
    }
}

impl Extend<FuriString> for FuriString {
    fn extend<T: IntoIterator<Item = FuriString>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_string(&s));
    }
}

impl Extend<char> for FuriString {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        self.reserve(lower_bound);
        iterator.for_each(move |c| self.push(c));
    }
}

impl<'a> Extend<&'a char> for FuriString {
    fn extend<T: IntoIterator<Item = &'a char>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<'a> Extend<&'a str> for FuriString {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(s));
    }
}

impl<'a> Extend<&'a CStr> for FuriString {
    fn extend<T: IntoIterator<Item = &'a CStr>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_c_str(s));
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl Extend<Box<str>> for FuriString {
    fn extend<T: IntoIterator<Item = Box<str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(&s));
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a> Extend<Cow<'a, str>> for FuriString {
    fn extend<T: IntoIterator<Item = Cow<'a, str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(&s));
    }
}

impl FromIterator<FuriString> for FuriString {
    #[allow(tail_expr_drop_order)]
    fn from_iter<T: IntoIterator<Item = FuriString>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();

        // Because we're iterating over `FuriString`s, we can avoid at least one
        // allocation by getting the first string from the iterator and appending to it
        // all the subsequent strings.
        match iterator.next() {
            None => FuriString::new(),
            Some(mut buf) => {
                buf.extend(iterator);
                buf
            }
        }
    }
}

impl FromIterator<char> for FuriString {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a char> for FuriString {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a str> for FuriString {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a CStr> for FuriString {
    fn from_iter<T: IntoIterator<Item = &'a CStr>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl FromIterator<Box<str>> for FuriString {
    fn from_iter<T: IntoIterator<Item = Box<str>>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a> FromIterator<Cow<'a, str>> for FuriString {
    fn from_iter<T: IntoIterator<Item = Cow<'a, str>>>(iter: T) -> Self {
        let mut buf = FuriString::new();
        buf.extend(iter);
        buf
    }
}

impl Add<&str> for FuriString {
    type Output = FuriString;

    #[inline]
    fn add(mut self, rhs: &str) -> Self::Output {
        self.push_str(rhs);
        self
    }
}

impl AddAssign<&str> for FuriString {
    #[inline]
    fn add_assign(&mut self, rhs: &str) {
        self.push_str(rhs);
    }
}

impl PartialEq for FuriString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::furi_string_equal(self.0.as_ptr(), other.0.as_ptr()) }
    }
}

impl PartialEq<CStr> for FuriString {
    fn eq(&self, other: &CStr) -> bool {
        unsafe { sys::furi_string_equal_str(self.0.as_ptr(), other.as_ptr()) }
    }
}

impl PartialEq<FuriString> for CStr {
    fn eq(&self, other: &FuriString) -> bool {
        other.eq(self)
    }
}

impl PartialEq<str> for FuriString {
    fn eq(&self, other: &str) -> bool {
        self.to_bytes().eq(other.as_bytes())
    }
}

impl PartialEq<FuriString> for str {
    fn eq(&self, other: &FuriString) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&str> for FuriString {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<FuriString> for &str {
    fn eq(&self, other: &FuriString) -> bool {
        other.eq(*self)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl PartialEq<CString> for FuriString {
    fn eq(&self, other: &CString) -> bool {
        self.eq(other.as_c_str())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl PartialEq<FuriString> for CString {
    fn eq(&self, other: &FuriString) -> bool {
        other.eq(self.as_c_str())
    }
}

impl Ord for FuriString {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { sys::furi_string_cmp(self.0.as_ptr(), other.0.as_ptr()) } {
            ..=-1 => Ordering::Less,
            0 => Ordering::Equal,
            1.. => Ordering::Greater,
        }
    }
}

impl PartialOrd for FuriString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for FuriString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_c_str().hash(state);
    }
}

impl fmt::Debug for FuriString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        for c in self.chars_lossy() {
            f.write_char(c)?;
        }
        f.write_char('"')
    }
}

impl ufmt::uDebug for FuriString {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_char('"')?;
        for c in self.chars_lossy() {
            f.write_char(c)?;
        }
        f.write_char('"')
    }
}

impl fmt::Display for FuriString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.chars_lossy() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl ufmt::uDisplay for FuriString {
    #[inline]
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        for c in self.chars_lossy() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Write for FuriString {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        Ok(())
    }
}

impl ufmt::uWrite for FuriString {
    type Error = Infallible;

    #[inline]
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.push(c);
        Ok(())
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::*;

    #[test]
    fn invalid_utf8_is_replaced() {
        // The German word für encoded in ISO 8859-1.
        let d: [u8; 3] = [0x66, 0xfc, 0x72];

        // Construct an invalid string using the Flipper Zero SDK.
        let s = FuriString::new();
        for b in d {
            unsafe { sys::furi_string_push_back(s.0.as_ptr(), b as c_char) };
        }

        for (l, r) in s.chars_lossy().zip("f�r".chars()) {
            assert_eq!(l, r);
        }
    }
}
