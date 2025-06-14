//! Unix pipe.
//!
//! See the [`new`] function for documentation.

cfg_os_ext! {
use std::io::{IoSlice, IoSliceMut, Read, Write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd};
use std::process::{ChildStderr, ChildStdin, ChildStdout};
use std::io;
use std::os::fd::RawFd;

use crate::{event, Interest, Registry, Token};

/// Create a new non-blocking Unix pipe.
///
/// This is a wrapper around Unix's [`pipe(2)`] system call and can be used as
/// inter-process or thread communication channel.
///
/// This channel may be created before forking the process and then one end used
/// in each process, e.g. the parent process has the sending end to send command
/// to the child process.
///
/// [`pipe(2)`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/pipe.html
///
/// # Events
///
/// The [`Sender`] can be registered with [`WRITABLE`] interest to receive
/// [writable events], the [`Receiver`] with [`READABLE`] interest. Once data is
/// written to the `Sender` the `Receiver` will receive an [readable event].
///
/// In addition to those events, events will also be generated if the other side
/// is dropped. To check if the `Sender` is dropped you'll need to check
/// [`is_read_closed`] on events for the `Receiver`, if it returns true the
/// `Sender` is dropped. On the `Sender` end check [`is_write_closed`], if it
/// returns true the `Receiver` was dropped. Also see the second example below.
///
/// [`WRITABLE`]: Interest::WRITABLE
/// [writable events]: event::Event::is_writable
/// [`READABLE`]: Interest::READABLE
/// [readable event]: event::Event::is_readable
/// [`is_read_closed`]: event::Event::is_read_closed
/// [`is_write_closed`]: event::Event::is_write_closed
///
/// # Deregistering
///
/// Both `Sender` and `Receiver` will deregister themselves when dropped,
/// **iff** the file descriptors are not duplicated (via [`dup(2)`]).
///
/// [`dup(2)`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/dup.html
///
/// # Examples
///
/// Simple example that writes data into the sending end and read it from the
/// receiving end.
///
/// ```
/// use std::io::{self, Read, Write};
///
/// use mio::{Poll, Events, Interest, Token};
/// use mio::unix::pipe;
///
/// // Unique tokens for the two ends of the channel.
/// const PIPE_RECV: Token = Token(0);
/// const PIPE_SEND: Token = Token(1);
///
/// # fn main() -> io::Result<()> {
/// // Create our `Poll` instance and the `Events` container.
/// let mut poll = Poll::new()?;
/// let mut events = Events::with_capacity(8);
///
/// // Create a new pipe.
/// let (mut sender, mut receiver) = pipe::new()?;
///
/// // Register both ends of the channel.
/// poll.registry().register(&mut receiver, PIPE_RECV, Interest::READABLE)?;
/// poll.registry().register(&mut sender, PIPE_SEND, Interest::WRITABLE)?;
///
/// const MSG: &[u8; 11] = b"Hello world";
///
/// loop {
///     poll.poll(&mut events, None)?;
///
///     for event in events.iter() {
///         match event.token() {
///             PIPE_SEND => sender.write(MSG)
///                 .and_then(|n| if n != MSG.len() {
///                         // We'll consider a short write an error in this
///                         // example. NOTE: we can't use `write_all` with
///                         // non-blocking I/O.
///                         Err(io::ErrorKind::WriteZero.into())
///                     } else {
///                         Ok(())
///                     })?,
///             PIPE_RECV => {
///                 let mut buf = [0; 11];
///                 let n = receiver.read(&mut buf)?;
///                 println!("received: {:?}", &buf[0..n]);
///                 assert_eq!(n, MSG.len());
///                 assert_eq!(&buf, &*MSG);
///                 return Ok(());
///             },
///             _ => unreachable!(),
///         }
///     }
/// }
/// # }
/// ```
///
/// Example that receives an event once the `Sender` is dropped.
///
/// ```
/// # use std::io;
/// #
/// # use mio::{Poll, Events, Interest, Token};
/// # use mio::unix::pipe;
/// #
/// # const PIPE_RECV: Token = Token(0);
/// # const PIPE_SEND: Token = Token(1);
/// #
/// # fn main() -> io::Result<()> {
/// // Same setup as in the example above.
/// let mut poll = Poll::new()?;
/// let mut events = Events::with_capacity(8);
///
/// let (mut sender, mut receiver) = pipe::new()?;
///
/// poll.registry().register(&mut receiver, PIPE_RECV, Interest::READABLE)?;
/// poll.registry().register(&mut sender, PIPE_SEND, Interest::WRITABLE)?;
///
/// // Drop the sender.
/// drop(sender);
///
/// poll.poll(&mut events, None)?;
///
/// for event in events.iter() {
///     match event.token() {
///         PIPE_RECV if event.is_read_closed() => {
///             // Detected that the sender was dropped.
///             println!("Sender dropped!");
///             return Ok(());
///         },
///         _ => unreachable!(),
///     }
/// }
/// # unreachable!();
/// # }
/// ```
pub fn new() -> io::Result<(Sender, Receiver)> {
    todo!()
}

/// Sending end of an Unix pipe.
///
/// See [`new`] for documentation, including examples.
#[derive(Debug)]
pub struct Sender;

impl Sender {
    /// Set the `Sender` into or out of non-blocking mode.
    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
    todo!()
    }

    /// Execute an I/O operation ensuring that the socket receives more events
    /// if it hits a [`WouldBlock`] error.
    ///
    /// # Notes
    ///
    /// This method is required to be called for **all** I/O operations to
    /// ensure the user will receive events once the socket is ready again after
    /// returning a [`WouldBlock`] error.
    ///
    /// [`WouldBlock`]: io::ErrorKind::WouldBlock
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::io;
    /// use std::os::fd::AsRawFd;
    /// use mio::unix::pipe;
    ///
    /// let (sender, receiver) = pipe::new()?;
    ///
    /// // Wait until the sender is writable...
    ///
    /// // Write to the sender using a direct libc call, of course the
    /// // `io::Write` implementation would be easier to use.
    /// let buf = b"hello";
    /// let n = sender.try_io(|| {
    ///     let buf_ptr = &buf as *const _ as *const _;
    ///     let res = unsafe { libc::write(sender.as_raw_fd(), buf_ptr, buf.len()) };
    ///     if res != -1 {
    ///         Ok(res as usize)
    ///     } else {
    ///         // If EAGAIN or EWOULDBLOCK is set by libc::write, the closure
    ///         // should return `WouldBlock` error.
    ///         Err(io::Error::last_os_error())
    ///     }
    /// })?;
    /// eprintln!("write {} bytes", n);
    ///
    /// // Wait until the receiver is readable...
    ///
    /// // Read from the receiver using a direct libc call, of course the
    /// // `io::Read` implementation would be easier to use.
    /// let mut buf = [0; 512];
    /// let n = receiver.try_io(|| {
    ///     let buf_ptr = &mut buf as *mut _ as *mut _;
    ///     let res = unsafe { libc::read(receiver.as_raw_fd(), buf_ptr, buf.len()) };
    ///     if res != -1 {
    ///         Ok(res as usize)
    ///     } else {
    ///         // If EAGAIN or EWOULDBLOCK is set by libc::read, the closure
    ///         // should return `WouldBlock` error.
    ///         Err(io::Error::last_os_error())
    ///     }
    /// })?;
    /// eprintln!("read {} bytes", n);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_io<F, T>(&self, _f: F) -> io::Result<T>
    where
        F: FnOnce() -> io::Result<T>,
    {
    todo!()
    }
}

impl event::Source for Sender {
    fn register(
        &mut self,
        _registry: &Registry,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<()> {
    todo!()
    }

    fn reregister(
        &mut self,
        _registry: &Registry,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<()> {
    todo!()
    }

    fn deregister(&mut self, _registry: &Registry) -> io::Result<()> {
        todo!()
    }
}

impl Write for Sender {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
    todo!()
    }

    fn write_vectored(&mut self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
    todo!()
    }
}

impl Write for &Sender {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
    todo!()
    }

    fn write_vectored(&mut self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
    todo!()
    }
}

/// # Notes
///
/// The underlying pipe is **not** set to non-blocking.
impl From<ChildStdin> for Sender {
    fn from(_stdin: ChildStdin) -> Sender {
    todo!()
    }
}

impl FromRawFd for Sender {
    unsafe fn from_raw_fd(_fd: RawFd) -> Sender {
    todo!()
    }
}

impl AsRawFd for Sender {
    fn as_raw_fd(&self) -> RawFd {
    todo!()
    }
}

impl IntoRawFd for Sender {
    fn into_raw_fd(self) -> RawFd {
    todo!()
    }
}

impl From<Sender> for OwnedFd {
    fn from(_sender: Sender) -> Self {
    todo!()
    }
}

impl AsFd for Sender {
    fn as_fd(&self) -> BorrowedFd<'_> {
    todo!()
    }
}

impl From<OwnedFd> for Sender {
    fn from(_fd: OwnedFd) -> Self {
    todo!()
    }
}

/// Receiving end of an Unix pipe.
///
/// See [`new`] for documentation, including examples.
#[derive(Debug)]
pub struct Receiver;

impl Receiver {
    /// Set the `Receiver` into or out of non-blocking mode.
    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
    todo!()
    }

    /// Execute an I/O operation ensuring that the socket receives more events
    /// if it hits a [`WouldBlock`] error.
    ///
    /// # Notes
    ///
    /// This method is required to be called for **all** I/O operations to
    /// ensure the user will receive events once the socket is ready again after
    /// returning a [`WouldBlock`] error.
    ///
    /// [`WouldBlock`]: io::ErrorKind::WouldBlock
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use std::io;
    /// use std::os::fd::AsRawFd;
    /// use mio::unix::pipe;
    ///
    /// let (sender, receiver) = pipe::new()?;
    ///
    /// // Wait until the sender is writable...
    ///
    /// // Write to the sender using a direct libc call, of course the
    /// // `io::Write` implementation would be easier to use.
    /// let buf = b"hello";
    /// let n = sender.try_io(|| {
    ///     let buf_ptr = &buf as *const _ as *const _;
    ///     let res = unsafe { libc::write(sender.as_raw_fd(), buf_ptr, buf.len()) };
    ///     if res != -1 {
    ///         Ok(res as usize)
    ///     } else {
    ///         // If EAGAIN or EWOULDBLOCK is set by libc::write, the closure
    ///         // should return `WouldBlock` error.
    ///         Err(io::Error::last_os_error())
    ///     }
    /// })?;
    /// eprintln!("write {} bytes", n);
    ///
    /// // Wait until the receiver is readable...
    ///
    /// // Read from the receiver using a direct libc call, of course the
    /// // `io::Read` implementation would be easier to use.
    /// let mut buf = [0; 512];
    /// let n = receiver.try_io(|| {
    ///     let buf_ptr = &mut buf as *mut _ as *mut _;
    ///     let res = unsafe { libc::read(receiver.as_raw_fd(), buf_ptr, buf.len()) };
    ///     if res != -1 {
    ///         Ok(res as usize)
    ///     } else {
    ///         // If EAGAIN or EWOULDBLOCK is set by libc::read, the closure
    ///         // should return `WouldBlock` error.
    ///         Err(io::Error::last_os_error())
    ///     }
    /// })?;
    /// eprintln!("read {} bytes", n);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_io<F, T>(&self, _f: F) -> io::Result<T>
    where
        F: FnOnce() -> io::Result<T>,
    {
    todo!()
    }
}

impl event::Source for Receiver {
    fn register(
        &mut self,
        _registry: &Registry,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<()> {
    todo!()
    }

    fn reregister(
        &mut self,
        _registry: &Registry,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<()> {
    todo!()
    }

    fn deregister(&mut self, _registry: &Registry) -> io::Result<()> {
    todo!()
    }
}

impl Read for Receiver {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
    todo!()
    }

    fn read_vectored(&mut self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    todo!()
    }
}

impl Read for &Receiver {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
    todo!()
    }

    fn read_vectored(&mut self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    todo!()
    }
}

/// # Notes
///
/// The underlying pipe is **not** set to non-blocking.
impl From<ChildStdout> for Receiver {
    fn from(_stdout: ChildStdout) -> Receiver {
        todo!();
    }
}

/// # Notes
///
/// The underlying pipe is **not** set to non-blocking.
impl From<ChildStderr> for Receiver {
    fn from(_stderr: ChildStderr) -> Receiver {
    todo!()
    }
}

impl IntoRawFd for Receiver {
    fn into_raw_fd(self) -> RawFd {
    todo!()
    }
}

impl AsRawFd for Receiver {
    fn as_raw_fd(&self) -> RawFd {
    todo!()
    }
}

impl FromRawFd for Receiver {
    unsafe fn from_raw_fd(_fd: RawFd) -> Receiver {
    todo!()
    }
}

impl From<Receiver> for OwnedFd {
    fn from(_receiver: Receiver) -> Self {
        todo!()
    }
}

impl AsFd for Receiver {
    fn as_fd(&self) -> BorrowedFd<'_> {
    todo!()
    }
}

impl From<OwnedFd> for Receiver {
    fn from(_fd: OwnedFd) -> Self {
    todo!()
    }
}

} // `cfg_os_ext!`.
