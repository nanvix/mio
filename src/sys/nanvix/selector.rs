// This implementation is based on the one in the `polling` crate.
// Thanks to https://github.com/Kestrer for the original implementation!
// Permission to use this code has been granted by original author:
// https://github.com/tokio-rs/mio/pull/1602#issuecomment-1218441031

use std::fmt::Debug;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::sync::Arc;
use std::time::Duration;

use crate::{Interest, Token};

#[derive(Debug)]
pub struct Selector {
    state: Arc<SelectorState>,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let state = SelectorState::new()?;

        Ok(Selector {
            state: Arc::new(state),
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        let state = self.state.clone();

        Ok(Selector { state })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        self.state.select(events, timeout)
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        self.state.register(fd, token, interests)
    }

    #[allow(dead_code)]
    pub(crate) fn register_internal(
        &self,
        fd: RawFd,
        token: Token,
        interests: Interest,
    ) -> io::Result<Arc<RegistrationRecord>> {
        self.state.register_internal(fd, token, interests)
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        self.state.reregister(fd, token, interests)
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        self.state.deregister(fd)
    }

    pub fn wake(&self, token: Token) -> io::Result<()> {
        self.state.wake(token)
    }

    cfg_io_source! {
        #[cfg(debug_assertions)]
        pub fn id(&self) -> usize {
            todo!();
        }
    }
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        todo!()
    }
}

/// Interface to poll.
#[derive(Debug)]
struct SelectorState;

impl SelectorState {
    pub fn new() -> io::Result<SelectorState> {
        todo!()
    }

    pub fn select(&self, _events: &mut Events, _timeout: Option<Duration>) -> io::Result<()> {
        todo!()
    }

    pub fn register(&self, _fd: RawFd, _token: Token, _interests: Interest) -> io::Result<()> {
        todo!()
    }

    pub fn register_internal(
        &self,
        _fd: RawFd,
        _token: Token,
        _interests: Interest,
    ) -> io::Result<Arc<RegistrationRecord>> {
        todo!()
    }

    pub fn reregister(&self, _fd: RawFd, _token: Token, _interests: Interest) -> io::Result<()> {
        todo!()
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        self.deregister_all(&[fd])
            .map_err(|_| io::ErrorKind::NotFound)?;
        Ok(())
    }

    /// Special optimized version of [Self::deregister] which handles multiple removals
    /// at once.  Ok result if all removals were performed, Err if any entries
    /// were not found.
    fn deregister_all(&self, _targets: &[RawFd]) -> Result<(), ()> {
        todo!()
    }

    pub fn wake(&self, _token: Token) -> io::Result<()> {
        todo!()
    }
}

/// Shared record between IoSourceState and SelectorState that allows us to internally
/// deregister partially or fully closed fds (i.e. when we get POLLHUP or PULLERR) without
/// confusing IoSourceState and trying to deregister twice.  This isn't strictly
/// required as technically deregister is idempotent but it is confusing
/// when trying to debug behaviour as we get imbalanced calls to register/deregister and
/// superfluous NotFound errors.
#[derive(Debug)]
pub(crate) struct RegistrationRecord;

impl RegistrationRecord {
    pub fn mark_unregistered(&self) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn is_registered(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Event;

pub type Events = Vec<Event>;

pub mod event {
    use std::fmt;

    use crate::sys::Event;
    use crate::Token;

    pub fn token(_event: &Event) -> Token {
        todo!()
    }

    pub fn is_readable(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_writable(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_error(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_read_closed(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_write_closed(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_priority(_event: &Event) -> bool {
        todo!()
    }

    pub fn is_aio(_: &Event) -> bool {
        todo!()
    }

    pub fn is_lio(_: &Event) -> bool {
        todo!()
    }

    pub fn debug_details(_f: &mut fmt::Formatter<'_>, _event: &Event) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct Waker {
    selector: Selector,
    token: Token,
}

impl Waker {
    pub(crate) fn new(selector: &Selector, token: Token) -> io::Result<Waker> {
        Ok(Waker {
            selector: selector.try_clone()?,
            token,
        })
    }

    pub(crate) fn wake(&self) -> io::Result<()> {
        self.selector.wake(self.token)
    }
}

cfg_io_source! {
    use crate::Registry;

    struct InternalState {
        selector: Selector,
        token: Token,
        interests: Interest,
        fd: RawFd,
        shared_record: Arc<RegistrationRecord>,
    }

    impl Drop for InternalState {
        fn drop(&mut self) {
            if self.shared_record.is_registered() {
                let _ = self.selector.deregister(self.fd);
            }
        }
    }

    pub(crate) struct IoSourceState {
        inner: Option<Box<InternalState>>,
    }

    impl IoSourceState {
        pub fn new() -> IoSourceState {
            IoSourceState { inner: None }
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
        F: FnOnce(&T) -> io::Result<R>,
        {
            let result = f(io);

            if let Err(err) = &result {
                if err.kind() == io::ErrorKind::WouldBlock {
                    self.inner.as_ref().map_or(Ok(()), |state| {
                        state
                        .selector
                        .reregister(state.fd, state.token, state.interests)
                    })?;
                }
            }

            result
        }

        pub fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            if self.inner.is_some() {
                Err(io::ErrorKind::AlreadyExists.into())
            } else {
                let selector = registry.selector().try_clone()?;

                selector.register_internal(fd, token, interests).map(move |shared_record| {
                    let state = InternalState {
                        selector,
                        token,
                        interests,
                        fd,
                        shared_record,
                    };

                    self.inner = Some(Box::new(state));
                })
            }
        }

        pub fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            match self.inner.as_mut() {
                Some(state) => registry
                .selector()
                .reregister(fd, token, interests)
                .map(|()| {
                    state.token = token;
                    state.interests = interests;
                }),
                None => Err(io::ErrorKind::NotFound.into()),
            }
        }

        pub fn deregister(&mut self, registry: &Registry, fd: RawFd) -> io::Result<()> {
            if let Some(state) = self.inner.take() {
                // Marking unregistered will short circuit the drop behaviour of calling
                // deregister so the call to deregister below is strictly required.
                state.shared_record.mark_unregistered();
            }

            registry.selector().deregister(fd)
        }
    }
}
