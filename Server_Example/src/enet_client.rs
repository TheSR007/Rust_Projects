use std::net::Ipv4Addr;
use anyhow::Context;
use enet::*;
use rocket::tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let enet = Enet::new().context("could not initialize ENet")?;
    let mut host = enet
        .create_host::<()>(
            None,
            1,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )
        .context("could not create host")?;
    
    host.connect(&Address::new(Ipv4Addr::LOCALHOST, 1234), 10, 0)
        .context("connect failed")?;
    
    let mut peer = loop {
        let e = host.service(1000).context("service failed")?;
        let e = match e {
            Some(ev) => ev,
            _ => continue,
        };
        println!("[client] event: {:#?}", e);
        match e {
            Event::Connect(ref p) => {
                break p.clone();
            }
            Event::Disconnect(ref p, r) => {
                println!("connection NOT successful, peer: {:?}, reason: {}", p, r);
                std::process::exit(0);
            }
            Event::Receive { .. } => {
                anyhow::bail!("unexpected Receive-event while waiting for connection")
            }
        };
    };

    // send a "hello" packet
    peer.send_packet(
        Packet::new(b"Client Hello", PacketMode::ReliableSequenced).unwrap(),
        1,
    )
    .context("sending packet failed")?;

    // disconnect after all outgoing packets have been sent.
    peer.disconnect_later(5);

    loop {
        let e = host.service(1000).context("service failed")?;
        println!("received event: {:#?}", e);
    
        match e {
            Some(Event::Disconnect(_, _)) => {
                println!("Disconnected from server. Exiting...");
                break Ok(());
            }
            _ => tokio::task::yield_now().await,
        }
    }
    
}