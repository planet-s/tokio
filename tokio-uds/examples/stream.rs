#![cfg(any(unix, target_os = "redox"))]

extern crate futures;
extern crate tokio;
extern crate tokio_uds;

extern crate tempdir;

use tokio_uds::*;

use tokio::io;
use tokio::runtime::current_thread::Runtime;

use futures::{Future, Stream};
use futures::sync::oneshot;
use tempdir::TempDir;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
    })
}

fn main() {
    println!("Hello World");

    let dir = TempDir::new("tokio-uds-tests").unwrap();
    let sock_path = dir.path().join("connect.sock");

    println!("{}", sock_path.display());

    let mut rt = Runtime::new().unwrap();

    let server = t!(UnixListener::bind(&sock_path));
    let (tx, rx) = oneshot::channel();

    rt.spawn({
        server.incoming()
            .into_future()
            .and_then(move |(sock, _)| {
                println!("Got client!");
                tx.send(sock.unwrap()).unwrap();
                Ok(())
            })
            .map_err(|e| panic!("err={:?}", e))
    });

    let client = rt.block_on(UnixStream::connect(&sock_path)).unwrap();
    let server = rt.block_on(rx).unwrap();

    // Write to the client
    rt.block_on(io::write_all(client, b"hello")).unwrap();
    println!("Wrote!");

    // Read from the server
    let (_, buf) = rt.block_on(io::read_to_end(server, vec![])).unwrap();
    println!("Read: {:?}", buf);

    assert_eq!(buf, b"hello");
    println!("hi");
}
