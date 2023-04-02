mod agent;

use crate::agent::agent::Agent;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::SocketAddrV4;

fn main() -> anyhow::Result<()> {
    let server = SocketAddrV4::new("127.0.0.1".parse()?, 0);
    let server = SockAddr::from(server);

    let sock = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(155)))?;

    let agent = Agent { server, sock };

    agent.online()?;

    agent.handle()?;

    Ok(())
}
