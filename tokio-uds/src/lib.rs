#![cfg(any(unix, target_os = "redox"))]
#![doc(html_root_url = "https://docs.rs/tokio-uds/0.2.4")]
#![deny(missing_docs, warnings, missing_debug_implementations)]

//! Unix Domain Sockets for Tokio.
//!
//! This crate provides APIs for using Unix Domain Sockets with Tokio.

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate iovec;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mio;
extern crate mio_uds;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_reactor;

#[cfg(not(target_os = "redox"))]
mod datagram;
#[cfg(not(target_os = "redox"))]
mod frame;
#[cfg(not(target_os = "redox"))]
mod recv_dgram;
#[cfg(not(target_os = "redox"))]
mod send_dgram;
#[cfg(not(target_os = "redox"))]
mod ucred;

mod incoming;
mod listener;
mod stream;

#[cfg(not(target_os = "redox"))]
pub use datagram::UnixDatagram;
#[cfg(not(target_os = "redox"))]
pub use frame::UnixDatagramFramed;
#[cfg(not(target_os = "redox"))]
pub use recv_dgram::RecvDgram;
#[cfg(not(target_os = "redox"))]
pub use send_dgram::SendDgram;
#[cfg(not(target_os = "redox"))]
pub use ucred::UCred;

pub use incoming::Incoming;
pub use listener::UnixListener;
pub use stream::{UnixStream, ConnectFuture};
