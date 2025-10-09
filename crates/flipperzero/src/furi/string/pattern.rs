//! The string Pattern API.
//!
//! The Pattern API provides a generic mechanism for using different pattern types when
//! searching through a string.
//!
//! For more details, see the [`Pattern`] trait.

use core::ffi::{CStr, c_char};

use flipperzero_sys as sys;

use super::FuriString;

const FURI_STRING_FAILURE: usize = usize::MAX;

/// A string pattern.
///
/// A `Pattern` expresses that the implementing type can be used as a string pattern for
/// searching in a [`FuriString`].
///
/// For example, both `'a'` and `"aa"` are patterns that would match at index `1` in the
/// string `"baaaab"`.
///
/// Depending on the type of the pattern, the behaviour of methods like
/// [`FuriString::find`] and [`FuriString::contains`] can change. The table below
/// describes some of those behaviours.
///
/// | Pattern type             | Match condition                           |
/// |--------------------------|-------------------------------------------|
/// | `&FuriString             | is substring                              |
/// | `c_char`                 | is contained in string                    |
/// | `&CStr                   | is substring                              |
/// | `char`                   | is contained in string                    |
/// | `&[char]`                | any char in slice is contained in string  |
pub trait Pattern: Sized {
    /// Checks whether the pattern matches anywhere in the haystack.
    #[inline]
    #[allow(clippy::wrong_self_convention)] // same as in `std`
    fn is_contained_in(self, haystack: &FuriString) -> bool {
        self.find_in(haystack).is_some()
    }

    /// Checks whether the pattern matches at the front of the haystack.
    #[allow(clippy::wrong_self_convention)] // same as in `std`
    fn is_prefix_of(self, haystack: &FuriString) -> bool;

    /// Checks whether the pattern matches at the back of the haystack.
    #[allow(clippy::wrong_self_convention)] // same as in `std`
    fn is_suffix_of(self, haystack: &FuriString) -> bool;

    /// Returns the byte index of the first byte of the haystack that matches the pattern.
    fn find_in(self, haystack: &FuriString) -> Option<usize>;

    /// Returns the byte index for the first byte of the last match of the pattern in the
    /// haystack.
    fn rfind_in(self, haystack: &FuriString) -> Option<usize>;

    /// Removes the pattern from the front of haystack, if it matches.
    #[must_use]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool;

    /// Removes the pattern from the back of haystack, if it matches.
    #[must_use]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool;
}

/// Searches for bytes that are equal to a given [`FuriString`].
impl Pattern for &FuriString {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_start_with(haystack.0.as_ptr(), self.0.as_ptr()) }
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_end_with(haystack.0.as_ptr(), self.0.as_ptr()) }
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        match unsafe { sys::furi_string_search(haystack.0.as_ptr(), self.0.as_ptr(), 0) } {
            FURI_STRING_FAILURE => None,
            i => Some(i),
        }
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        // TODO: Replace with a more efficient strategy.
        let haystack = haystack.to_bytes();
        let needle = self.to_bytes();
        if haystack.len() >= needle.len() {
            for start in (haystack.len().saturating_sub(needle.len() + 1))..=0 {
                if haystack[start..].starts_with(needle) {
                    return Some(start);
                }
            }
        }
        None
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        let is_prefix = self.is_prefix_of(haystack);
        if is_prefix {
            unsafe { sys::furi_string_right(haystack.0.as_ptr(), self.len()) };
        }
        is_prefix
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        let is_suffix = self.is_suffix_of(haystack);
        if is_suffix {
            haystack.truncate(haystack.len() - self.len());
        }
        is_suffix
    }
}

/// Searches for bytes that are equal to a given [`c_char`].
impl Pattern for c_char {
    #[inline]
    fn is_contained_in(self, haystack: &FuriString) -> bool {
        // `c_char` may `i8` in certain configurations
        #[allow(clippy::unnecessary_cast)]
        haystack.to_bytes().contains(&(self as u8))
    }

    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_start_with_str(haystack.0.as_ptr(), [self, 0].as_ptr()) }
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_end_with_str(haystack.0.as_ptr(), [self, 0].as_ptr()) }
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        match unsafe { sys::furi_string_search_char(haystack.0.as_ptr(), self, 0) } {
            FURI_STRING_FAILURE => None,
            i => Some(i),
        }
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        match unsafe { sys::furi_string_search_rchar(haystack.0.as_ptr(), self, 0) } {
            FURI_STRING_FAILURE => None,
            i => Some(i),
        }
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        let is_prefix = self.is_prefix_of(haystack);
        if is_prefix {
            unsafe { sys::furi_string_right(haystack.0.as_ptr(), 1) };
        }
        is_prefix
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        let is_suffix = self.is_suffix_of(haystack);
        if is_suffix {
            haystack.truncate(haystack.len() - 1);
        }
        is_suffix
    }
}

/// Searches for bytes that are equal to a given [`CStr`].
impl Pattern for &CStr {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_start_with_str(haystack.0.as_ptr(), self.as_ptr()) }
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        unsafe { sys::furi_string_end_with_str(haystack.0.as_ptr(), self.as_ptr()) }
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        match unsafe { sys::furi_string_search_str(haystack.0.as_ptr(), self.as_ptr(), 0) } {
            FURI_STRING_FAILURE => None,
            i => Some(i),
        }
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        // TODO: Replace with a more efficient strategy.
        let haystack = haystack.to_bytes();
        let needle = self.to_bytes();
        if haystack.len() >= needle.len() {
            for start in (0..(haystack.len().saturating_sub(needle.len()))).rev() {
                if haystack[start..].starts_with(needle) {
                    return Some(start);
                }
            }
        }
        None
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        let is_prefix = self.is_prefix_of(haystack);
        if is_prefix {
            unsafe { sys::furi_string_right(haystack.0.as_ptr(), self.to_bytes().len()) };
        }
        is_prefix
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        let is_suffix = self.is_suffix_of(haystack);
        if is_suffix {
            haystack.truncate(haystack.len() - self.to_bytes().len());
        }
        is_suffix
    }
}

/// Runs the given `CStr`-taking closure over a `char`.
///
/// # Safety
///
/// `c` must not be the NULL character.
#[inline]
fn with_char_as_cstr<T>(c: char, f: impl FnOnce(&CStr) -> T) -> T {
    // Use a buffer of length 5 to transform the `char` into a C string.
    let mut buffer = [0; 5];
    let len = c.encode_utf8(&mut buffer).len();
    // SAFETY: `self` is not the NULL character, and `buffer[len]` is guaranteed to be a
    // valid nul-terminating byte (because `buffer` is zero-initialized and `len <= 4`).
    let needle = unsafe { CStr::from_bytes_with_nul_unchecked(&buffer[..len + 1]) };
    f(needle)
}

/// Searches for bytes that are equal to a given [`char`].
impl Pattern for char {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        if (self as u32) < 128 {
            (self as c_char).is_prefix_of(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.is_prefix_of(haystack))
        }
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        if (self as u32) < 128 {
            (self as c_char).is_suffix_of(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.is_suffix_of(haystack))
        }
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        if (self as u32) < 128 {
            (self as c_char).find_in(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.find_in(haystack))
        }
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        if (self as u32) < 128 {
            (self as c_char).rfind_in(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.rfind_in(haystack))
        }
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        if (self as u32) < 128 {
            (self as c_char).strip_prefix_of(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.strip_prefix_of(haystack))
        }
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        if (self as u32) < 128 {
            (self as c_char).strip_suffix_of(haystack)
        } else {
            with_char_as_cstr(self, |needle| needle.strip_suffix_of(haystack))
        }
    }
}

/// Searches for bytes that are equal to any of the [`char`]s in the slice.
impl Pattern for &[char] {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        self.iter().any(|c| c.is_prefix_of(haystack))
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        self.iter().any(|c| c.is_suffix_of(haystack))
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        // TODO: Replace with a more efficient strategy.
        self.iter()
            .map(|c| c.find_in(haystack))
            .fold(None, |acc, res| match (acc, res) {
                (None, _) => res,
                (Some(a), Some(b)) if a > b => res,
                (Some(_), _) => acc,
            })
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        // TODO: Replace with a more efficient strategy.
        self.iter()
            .map(|c| c.find_in(haystack))
            .fold(None, |acc, res| match (acc, res) {
                (None, _) => res,
                (Some(a), Some(b)) if a < b => res,
                (Some(_), _) => acc,
            })
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        self.iter().any(|c| c.strip_prefix_of(haystack))
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        self.iter().any(|c| c.strip_suffix_of(haystack))
    }
}

/// Searches for bytes that are equal to any of the [`char`]s in the array.
impl<const N: usize> Pattern for [char; N] {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        self[..].is_prefix_of(haystack)
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        self[..].is_suffix_of(haystack)
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        self[..].find_in(haystack)
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        self[..].rfind_in(haystack)
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        self[..].strip_prefix_of(haystack)
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        self[..].strip_suffix_of(haystack)
    }
}

/// Searches for bytes that are equal to any of the [`char`]s in the array.
impl<const N: usize> Pattern for &[char; N] {
    #[inline]
    fn is_prefix_of(self, haystack: &FuriString) -> bool {
        self[..].is_prefix_of(haystack)
    }

    #[inline]
    fn is_suffix_of(self, haystack: &FuriString) -> bool {
        self[..].is_suffix_of(haystack)
    }

    #[inline]
    fn find_in(self, haystack: &FuriString) -> Option<usize> {
        self[..].find_in(haystack)
    }

    #[inline]
    fn rfind_in(self, haystack: &FuriString) -> Option<usize> {
        self[..].rfind_in(haystack)
    }

    #[inline]
    fn strip_prefix_of(self, haystack: &mut FuriString) -> bool {
        self[..].strip_prefix_of(haystack)
    }

    #[inline]
    fn strip_suffix_of(self, haystack: &mut FuriString) -> bool {
        self[..].strip_suffix_of(haystack)
    }
}
