use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, RawFd};

use crate::sys::Selector;
use crate::Token;

#[derive(Debug)]
pub(crate) struct Waker {
    fd: File,
}

impl Waker {
    #[allow(dead_code)] // Not used by the `poll(2)` implementation.
    pub(crate) fn new(_selector: &Selector, _token: Token) -> io::Result<Waker> {
        todo!()
    }

    #[allow(dead_code)] // Only used by the `poll(2)` implementation.
    pub(crate) fn ack_and_reset(&self) {
        let _ = self.reset();
    }

    /// Reset the eventfd object, only need to call this if `wake` fails.
    #[allow(clippy::unused_io_amount)] // Don't care about partial reads.
    fn reset(&self) -> io::Result<()> {
        todo!()
    }
}

impl AsRawFd for Waker {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}
