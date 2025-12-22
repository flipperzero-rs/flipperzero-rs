use flipperzero_sys as sys;
use rand_core::{CryptoRng, Error, RngCore, impls};

/// A random number generator that retrieves randomness from the Flipper Zero hardware.
///
/// This is a zero-sized struct. It can be freely constructed with `HwRng`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HwRng;

impl CryptoRng for HwRng {}

impl RngCore for HwRng {
    fn next_u32(&mut self) -> u32 {
        unsafe { sys::furi_hal_random_get() }
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        unsafe { sys::furi_hal_random_fill_buf(dest.as_mut_ptr(), dest.len() as u32) };
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[flipperzero_test::tests]
mod tests {
    use rand_core::RngCore;

    use super::HwRng;

    #[test]
    fn test_hw_rng() {
        let x = HwRng.next_u64();
        let y = HwRng.next_u64();
        assert!(x != 0);
        assert!(x != y);
    }

    #[test]
    fn test_construction() {
        let mut rng = HwRng::default();
        assert!(rng.next_u64() != 0);
    }
}
