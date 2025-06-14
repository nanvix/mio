use std::io;
use std::net::{self, SocketAddr};
use std::os::nanvix::syscall::ffi::c_int;

pub(crate) fn new_for_addr(_address: SocketAddr) -> io::Result<c_int> {
    todo!();
}

pub(crate) fn bind(_socket: &net::TcpListener, _addr: SocketAddr) -> io::Result<()> {
    todo!()
}

pub(crate) fn connect(_socket: &net::TcpStream, _addr: SocketAddr) -> io::Result<()> {
    todo!()
}

pub(crate) fn listen(_socket: &net::TcpListener, _backlog: u32) -> io::Result<()> {
    todo!()
}

pub(crate) fn set_reuseaddr(_socket: &net::TcpListener, _reuseaddr: bool) -> io::Result<()> {
    todo!()
}

pub(crate) fn accept(_listener: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
    todo!()
}
