//! User-friendly wrappers of XBM images.

use crate::internals::ops::div_ceil_u16;
use alloc::{vec, vec::Vec};
use core::{
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct XbmImage<D> {
    data: D,
    width: u16,
    height: u16,
}

impl<D> XbmImage<D> {
    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub const fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    #[inline]
    const fn row_bytes(width: u16) -> u16 {
        div_ceil_u16(width, 8)
    }

    #[inline]
    const fn total_bytes(width: u16, height: u16) -> u32 {
        Self::row_bytes(width) as u32 * height as u32
    }

    // IMPORTANT: XBM images have trailing bits per-rows
    // rather than at the end of the whole byte-array
    #[inline]
    const fn offsets(&self, x: u16, y: u16) -> Option<(u16, u16)> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((
                // Per each y we skip a row of `row_bytes` bytes
                // then we also have to skip all previous
                (Self::row_bytes(self.width) * y) + x / 8,
                // Since all rows are aligned, only x affects the bit offset
                x % 8,
            ))
        }
    }
}

impl<T: ?Sized, D: Deref<Target = T>> XbmImage<D> {
    pub fn data(&self) -> &D::Target {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D::Target
    where
        D: DerefMut,
    {
        &mut self.data
    }
}

impl<D: Deref<Target = [u8]>> XbmImage<D> {
    pub fn new_from(width: u16, height: u16, data: D) -> Self {
        let bytes = Self::total_bytes(width, height);

        assert!(
            bytes as usize == data.len(),
            "width={}bits * height={}bits should correspond to {}bytes, but data has length {}",
            width,
            height,
            bytes,
            data.len()
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, (x, y): (u16, u16)) -> Option<bool> {
        if let Some((byte, shift)) = self.offsets(x, y) {
            Some((self.data[byte as usize] >> shift) & 0b1 != 0)
        } else {
            None
        }
    }
}

impl<'a> XbmImage<&'a [u8]> {
    pub unsafe fn from_raw(width: u16, height: u16, data: *const u8) -> Self {
        let bytes = Self::total_bytes(width, height) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts(data, bytes) };

        Self {
            data,
            width,
            height,
        }
    }
}

impl<'a> XbmImage<&'a mut [u8]> {
    pub unsafe fn from_raw_mut(width: u16, height: u16, data: *mut u8) -> Self {
        let bytes = Self::total_bytes(width, height) as usize;

        // SAFETY: the size is exactly calculated based on width and height
        // and caller upholds the `data` validity invariant
        let data = unsafe { slice::from_raw_parts_mut(data, bytes) };

        Self {
            data,
            width,
            height,
        }
    }
}

impl<D: Deref<Target = [u8]> + DerefMut> XbmImage<D> {
    pub fn set(&mut self, coordinates: (u16, u16), value: bool) -> Option<()> {
        if value {
            self.set_1(coordinates)
        } else {
            self.set_0(coordinates)
        }
    }

    pub fn set_1(&mut self, (x, y): (u16, u16)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] |= 1 << shift;
        Some(())
    }

    pub fn set_0(&mut self, (x, y): (u16, u16)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] &= !(1 << shift);
        Some(())
    }

    pub fn xor(&mut self, (x, y): (u16, u16)) -> Option<()> {
        let (byte, shift) = self.offsets(x, y)?;
        self.data[byte as usize] ^= 1 << shift;
        Some(())
    }
}

// Specializations

impl XbmImage<Vec<u8>> {
    pub fn new(width: u16, height: u16) -> Self {
        let bytes = Self::total_bytes(width, height) as usize;
        Self {
            data: vec![0; bytes],
            width,
            height,
        }
    }
}

impl XbmImage<&'static [u8]> {
    /// Creates a referencing `XbmImage` from a static `u8` slice.
    ///
    /// This is a constant specialization of [`XbmImage::new_from`]
    /// existing since the latter cannot be generically `const`
    /// until `const_fn_trait_bound` Rust feature becomes stable.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use flipperzero::xbm::XbmImage;
    /// const IMAGE: XbmImage<&'static [u8]> = XbmImage::new_from_static(4, 4, &[0xFE, 0x12]);
    /// ```
    pub const fn new_from_static(width: u16, height: u16, data: &'static [u8]) -> Self {
        let bytes = Self::total_bytes(width, height);

        assert!(
            bytes as usize == data.len(),
            "dimensions don't match data length",
        );

        Self {
            data,
            width,
            height,
        }
    }
}

impl<const SIZE: usize> XbmImage<ByteArray<SIZE>> {
    /// Creates a referencing `XbmImage` from a `u8` array.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use flipperzero::xbm::{XbmImage, ByteArray};
    /// const IMAGE: XbmImage<ByteArray<2>> = XbmImage::new_from_array::<4, 4>([0xFE, 0x12]);
    /// ```
    pub const fn new_from_array<const WIDTH: u16, const HEIGHT: u16>(data: [u8; SIZE]) -> Self {
        let bytes = Self::total_bytes(WIDTH, HEIGHT);

        assert!(bytes as usize == SIZE, "dimensions don't match data length");

        Self {
            data: ByteArray(data),
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ByteArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

/// Creates a compile-time XBM image.
/// The type of this expression is [`XbmImage`] with [`ByteArray`] backend of the calculated size.
///
/// The syntax is an [XBM image definition][XBM format]
/// optionally wrapped in an `unsafe` block.
///
/// Unless the expression is wrapped in an `unsafe` block
/// the macro will perform identifier validation:
/// it will check that identifiers are in the order
/// `<name>_width`, `<name>_height`, optionally `<name>_x_hot` and `<name>_y_hot` and `<name>_bits`
/// and that the `<name>` is the same for all identifiers.
/// The `unsafe` form omits this validations
/// while still ensuring that the size constraints are valid.
///
/// [XBM format]: https://www.fileformat.info/format/xbm/egff.htm
#[macro_export]
macro_rules! xbm {
    (
        #define $width_ident:ident $width:literal
        #define $height_ident:ident $height:literal
        $(
            #define $x_hotspot_ident:ident $_hotspot_x:literal
            #define $y_hotspot_ident:ident $_hotspot_y:literal
        )?
        static $(unsigned)? char $bits_ident:ident[] = {
            $($byte:literal),* $(,)?
        };
    ) => {{
        { // name assertions
            let bits_ident = stringify!($bits_ident).as_bytes();
            assert!(
                matches!(bits_ident, [.., b'_', b'b', b'i', b't', b's']),
                "width identifier should end with `_bits`",
            );
            let significant_len = bits_ident.len() - b"_bits".len();

            const fn str_eq(left: &[u8], right: &[u8], limit: usize) -> bool {
                match (left.split_first(), right.split_first()) {
                    (
                        Some((&left_first, left_remaining)),
                        Some((&right_first, right_remaining)),
                    ) => {
                        left_first == right_first
                            && (limit == 1 || str_eq(left_remaining, right_remaining, limit - 1))
                    }
                    (None, None) => true,
                    _ => false,
                }
            }

            let width_ident = stringify!($width_ident).as_bytes();
            assert!(
                matches!(width_ident, [.., b'_', b'w', b'i', b'd', b't', b'h']),
                "width identifier should end with `_width`",
            );
            assert!(
                str_eq(bits_ident, width_ident, significant_len),
                "bits identifier and width identifier should have the same prefix"
            );

            let height_ident = stringify!($height_ident).as_bytes();
            assert!(
                matches!(height_ident, [.., b'_', b'h', b'e', b'i', b'g', b'h', b't']),
                "width identifier should end with `_height`",
            );
            assert!(
                str_eq(bits_ident, height_ident, significant_len),
                "bits identifier and height identifier should have the same prefix"
            );

            $(
            let x_hotspot_ident = stringify!($x_hotspot_ident).as_bytes();
            assert!(
                matches!(bits_ident, [.., b'_', b'x', b'_', b'h', b'o', b't']),
                "x-hotspot identifier should end with `_x_hot`",
            );
            assert!(
                str_eq(bits_ident, x_hotspot_ident, significant_len),
                "bits identifier and x-hotspot identifier should have the same prefix"
            );

            let y_hotspot_ident = stringify!($y_hotspot_ident).as_bytes();
            assert!(
                matches!(bits_ident, [.., b'_', b'y', b'_', b'h', b'o', b't']),
                "y-hotspot identifier should end with `_y_hot`",
            );
            assert!(
                str_eq(bits_ident, y_hotspot_ident, significant_len),
                "bits identifier and y-hotspot identifier should have the same prefix"
            );
            )?
        }

        $crate::xbm!(unsafe {
            #define $width_ident $width
            #define $height_ident $height
            $(
                #define $x_hotspot_ident $_hotspot_x
                #define $y_hotspot_ident $_hotspot_y
            )?
            static char $bits_ident[] = {
                $($byte),*
            };
        })
    }};
    (
        unsafe {
            #define $_width_ident:ident $width:literal
            #define $_height_ident:ident $height:literal
            $(
                #define $_x_hotspot_ident:ident $_hotspot_x:literal
                #define $_y_hotspot_ident:ident $_hotspot_y:literal
            )?
            static $(unsigned)? char $_bits_ident:ident[] = {
                $($byte:literal),* $(,)?
            };
        }
    ) => {{
        $crate::gui::xbm::XbmImage::new_from_array::<$width, $height>([$($byte,)*])
    }};
}

#[flipperzero_test::tests]
mod tests {
    #[test]
    fn valid_byte_reading_aligned() {
        // 0100110000111100
        // 0000001111111100
        let image = xbm!(
            #define xbm_test_width 16
            #define xbm_test_height 2
            static char xbm_test_bits[] = {
                0x32, // 0b00110010 ~ 0b01001100
                0x3C, // 0b00111100 ~ 0b00111100
                0xC0, // 0b11000000 ~ 0b00000011
                0x3F, // 0b00111111 ~ 0b11111100
            };
        );

        assert!(!image.get((0, 0)).unwrap());
        assert!(image.get((1, 0)).unwrap());
        assert!(!image.get((2, 0)).unwrap());
        assert!(!image.get((3, 0)).unwrap());
        assert!(image.get((4, 0)).unwrap());
        assert!(image.get((5, 0)).unwrap());
        assert!(!image.get((6, 0)).unwrap());
        assert!(!image.get((7, 0)).unwrap());
        assert!(!image.get((8, 0)).unwrap());
        assert!(!image.get((9, 0)).unwrap());
        assert!(image.get((10, 0)).unwrap());
        assert!(image.get((11, 0)).unwrap());
        assert!(image.get((12, 0)).unwrap());
        assert!(image.get((13, 0)).unwrap());
        assert!(!image.get((14, 0)).unwrap());
        assert!(!image.get((15, 0)).unwrap());
        assert!(!image.get((0, 1)).unwrap());
        assert!(!image.get((1, 1)).unwrap());
        assert!(!image.get((2, 1)).unwrap());
        assert!(!image.get((3, 1)).unwrap());
        assert!(!image.get((4, 1)).unwrap());
        assert!(!image.get((5, 1)).unwrap());
        assert!(image.get((6, 1)).unwrap());
        assert!(image.get((7, 1)).unwrap());
        assert!(image.get((8, 1)).unwrap());
        assert!(image.get((9, 1)).unwrap());
        assert!(image.get((10, 1)).unwrap());
        assert!(image.get((11, 1)).unwrap());
        assert!(image.get((12, 1)).unwrap());
        assert!(image.get((13, 1)).unwrap());
        assert!(!image.get((14, 1)).unwrap());
        assert!(!image.get((15, 1)).unwrap());
    }

    #[test]
    fn valid_byte_reading_misaligned() {
        // 01001100 00111100 0*******
        // 00000011 11111100 1*******
        let image = xbm!(
            #define xbm_test_width 17
            #define xbm_test_height 2
            static char xbm_test_bits[] = {
                0x32, // 0b00110010 ~ 0b01001100
                0x3C, // 0b00111100 ~ 0b00111100
                0xF0, // 0b00001111 ~ 0b11110000
                0xC0, // 0b11000000 ~ 0b00000011
                0x3F, // 0b00111111 ~ 0b11111100
                0x0F, // 0b11110000 ~ 0b00001111
            };
        );

        assert!(!image.get((0, 0)).unwrap());
        assert!(image.get((1, 0)).unwrap());
        assert!(!image.get((2, 0)).unwrap());
        assert!(!image.get((3, 0)).unwrap());
        assert!(image.get((4, 0)).unwrap());
        assert!(image.get((5, 0)).unwrap());
        assert!(!image.get((6, 0)).unwrap());
        assert!(!image.get((7, 0)).unwrap());
        assert!(!image.get((8, 0)).unwrap());
        assert!(!image.get((9, 0)).unwrap());
        assert!(image.get((10, 0)).unwrap());
        assert!(image.get((11, 0)).unwrap());
        assert!(image.get((12, 0)).unwrap());
        assert!(image.get((13, 0)).unwrap());
        assert!(!image.get((14, 0)).unwrap());
        assert!(!image.get((15, 0)).unwrap());
        assert!(!image.get((16, 0)).unwrap());
        assert!(!image.get((0, 1)).unwrap());
        assert!(!image.get((1, 1)).unwrap());
        assert!(!image.get((2, 1)).unwrap());
        assert!(!image.get((3, 1)).unwrap());
        assert!(!image.get((4, 1)).unwrap());
        assert!(!image.get((5, 1)).unwrap());
        assert!(image.get((6, 1)).unwrap());
        assert!(image.get((7, 1)).unwrap());
        assert!(image.get((8, 1)).unwrap());
        assert!(image.get((9, 1)).unwrap());
        assert!(image.get((10, 1)).unwrap());
        assert!(image.get((11, 1)).unwrap());
        assert!(image.get((12, 1)).unwrap());
        assert!(image.get((13, 1)).unwrap());
        assert!(!image.get((14, 1)).unwrap());
        assert!(!image.get((15, 1)).unwrap());
        assert!(image.get((16, 1)).unwrap());
    }
}
