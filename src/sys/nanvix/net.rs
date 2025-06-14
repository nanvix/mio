use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// pub(crate) fn new_ip_socket(addr: SocketAddr, socket_type: libc::c_int) -> io::Result<libc::c_int> {
//     todo!()
// }

/// Create a new non-blocking socket.
// pub(crate) fn new_socket(domain: libc::c_int, socket_type: libc::c_int) -> io::Result<libc::c_int> {
//     todo!()
// }

/// A type with the same memory layout as `libc::sockaddr`. Used in converting Rust level
/// SocketAddr* types into their system representation. The benefit of this specific
/// type over using `libc::sockaddr_storage` is that this type is exactly as large as it
/// needs to be and not a lot larger. And it can be initialized cleaner from Rust.
// #[repr(C)]
// pub(crate) union SocketAddrCRepr {
//     v4: libc::sockaddr_in,
//     v6: libc::sockaddr_in6,
// }

impl SocketAddrCRepr {
    pub(crate) fn as_ptr(&self) -> *const libc::sockaddr {
        todo!()
    }
}

/// Converts a Rust `SocketAddr` into the system representation.
pub(crate) fn socket_addr(addr: &SocketAddr) -> (SocketAddrCRepr, libc::socklen_t) {
    todo!()
}

/// Converts a `libc::sockaddr` compatible struct into a native Rust `SocketAddr`.
///
/// # Safety
///
/// `storage` must have the `ss_family` field correctly initialized.
/// `storage` must be initialised to a `sockaddr_in` or `sockaddr_in6`.
pub(crate) unsafe fn to_socket_addr(
    storage: *const libc::sockaddr_storage,
) -> io::Result<SocketAddr> {
    todo!()
}
