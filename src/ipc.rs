//TODO: Swap out for ZBus

use std::ops::{Deref, DerefMut};
use std::os::fd::{FromRawFd, RawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use nix::sys::socket;
use nix::sys::socket::UnixAddr;

pub struct IPCServer {
    listener: UnixListener
}

impl IPCServer {
    /// Create a new IPC Server
    ///
    /// It can be assumed that a [`nix::errno::Errno::EADDRINUSE`] means that a server instance
    /// already exists
    ///
    /// # Arguments
    ///
    /// * `name`: The name of the abstract unix socket to bind the server to
    ///
    /// returns: [`Result`]<[`IPCServer`], [`nix::errno::Errno`]>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nix::errno::Errno;
    /// use dat_mod_manager::ipc::IPCServer;
    /// match IPCServer::new("my-abstract-socket-name.sock") {
    ///     Ok(ipcServer) => {
    ///         // We've got our server
    ///     },
    ///     Err(Errno::EADDRINUSE) => {
    ///         // A server already exists on this socket
    ///     },
    ///     Err(e) => {
    ///         panic!("An unexpected error occured")
    ///     }
    /// }
    /// ```
    pub fn new(name: &str) -> Result<Self, nix::errno::Errno> {
        let addr = UnixAddr::new_abstract(name.as_bytes())?;
        let sock = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::Stream,
            socket::SockFlag::empty(),
            None
        )?;

        const BACKLOG: usize =
            if cfg!(any(target_os = "linux", target_os = "freebsd")) { usize::MAX } else { 128 };

        socket::bind(sock, &addr)?;
        socket::listen(sock, BACKLOG)?;

        // Wrap it in a listener for nice listening
        // Assumes that all the errors that could happen have already happened
        let listener = unsafe {
            UnixListener::from_raw_fd(sock)
        };

        Ok(Self{listener})
    }
}

impl Deref for IPCServer {
    type Target = UnixListener;

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

pub struct IPCClient {
    stream: UnixStream
}

impl IPCClient {
    pub fn new(name: &str) -> Result<IPCClient, nix::errno::Errno> {
        let addr = UnixAddr::new_abstract(name.as_bytes())?;
        let sock = socket::socket(
            socket::AddressFamily::Unix,
            socket::SockType::Stream,
            socket::SockFlag::empty(),
            None
        )?;

        socket::connect(sock, &addr)?;

        // Wrap it in a stream for nice messaging
        // Assumes that all the errors that could happen have already happened
        let stream = unsafe {
            UnixStream::from_raw_fd(sock)
        };

        Ok(Self{stream})
    }
}