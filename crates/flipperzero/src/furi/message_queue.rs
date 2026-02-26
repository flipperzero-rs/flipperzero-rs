use core::ffi::c_void;
use core::ptr::{self, NonNull};

use flipperzero_sys as sys;
use flipperzero_sys::furi::Status;

use crate::furi;
use crate::furi::time::FuriDuration;

/// MessageQueue provides a safe wrapper around the furi message queue primitive.
pub struct MessageQueue<M: Sized> {
    hnd: NonNull<sys::FuriMessageQueue>,
    _marker: core::marker::PhantomData<M>,
}

impl<M: Sized> MessageQueue<M> {
    /// Constructs a message queue with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            hnd: unsafe {
                NonNull::new_unchecked(sys::furi_message_queue_alloc(
                    capacity as u32,
                    core::mem::size_of::<M>() as u32,
                ))
            },
            _marker: core::marker::PhantomData::<M>,
        }
    }

    // Attempts to add the message to the end of the queue, waiting up to timeout ticks.
    pub fn put(&self, msg: M, timeout: FuriDuration) -> furi::Result<()> {
        let mut msg = core::mem::ManuallyDrop::new(msg);

        let status: Status = unsafe {
            sys::furi_message_queue_put(
                self.hnd.as_ptr(),
                ptr::from_mut(&mut msg).cast::<c_void>(),
                timeout.as_ticks(),
            )
            .into()
        };

        let _ = status.into_result()?;

        Ok(())
    }

    // Attempts to read a message from the front of the queue within timeout ticks.
    pub fn get(&self, timeout: FuriDuration) -> furi::Result<M> {
        let mut out = core::mem::MaybeUninit::<M>::uninit();
        let status: Status = unsafe {
            sys::furi_message_queue_get(
                self.hnd.as_ptr(),
                out.as_mut_ptr().cast::<c_void>(),
                timeout.as_ticks(),
            )
            .into()
        };

        let _ = status.into_result()?;

        Ok(unsafe { out.assume_init() })
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> usize {
        unsafe { sys::furi_message_queue_get_capacity(self.hnd.as_ptr()) as usize }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        unsafe { sys::furi_message_queue_get_count(self.hnd.as_ptr()) as usize }
    }

    /// Is the message queue empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of free slots in the queue.
    pub fn space(&self) -> usize {
        unsafe { sys::furi_message_queue_get_space(self.hnd.as_ptr()) as usize }
    }
}

impl<M: Sized> Drop for MessageQueue<M> {
    fn drop(&mut self) {
        // Drain any elements from the message queue, so any
        // drop handlers on the message element get called.
        while !self.is_empty() {
            match self.get(FuriDuration::MAX) {
                Ok(msg) => drop(msg),
                Err(_) => break, // we tried
            }
        }

        unsafe { sys::furi_message_queue_free(self.hnd.as_ptr()) }
    }
}

#[flipperzero_test::tests]
mod tests {
    use super::*;

    use super::MessageQueue;

    #[test]
    fn capacity() {
        let queue = MessageQueue::new(3);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.space(), 3);
        assert_eq!(queue.capacity(), 3);

        // Adding a message to the queue should consume capacity.
        queue.put(2, FuriDuration::from_millis(1)).unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.space(), 2);
        assert_eq!(queue.capacity(), 3);

        // We should be able to fill the queue to capacity.
        queue.put(4, FuriDuration::from_millis(1)).unwrap();
        queue.put(6, FuriDuration::from_millis(1)).unwrap();
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.space(), 0);
        assert_eq!(queue.capacity(), 3);

        // Attempting to add another message should time out.
        assert_eq!(
            queue.put(7, FuriDuration::from_millis(1)),
            Err(furi::Error::TimedOut),
        );

        // Removing a message from the queue frees up capacity.
        assert_eq!(queue.get(FuriDuration::from_millis(1)), Ok(2));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.space(), 1);
        assert_eq!(queue.capacity(), 3);
    }
}
