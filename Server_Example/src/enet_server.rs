use anyhow::Context;
use enet::*;
use rocket::tokio;
use std::net::Ipv4Addr;
use tokio::sync::mpsc;
use tokio::time::Duration;

pub enum EnetCommand {
    Shutdown,
}

pub async fn run_enet_server(mut rx: mpsc::Receiver<EnetCommand>) -> anyhow::Result<()> {
    let enet = Enet::new().context("could not initialize ENet")?;
    let local_addr = Address::new(Ipv4Addr::LOCALHOST, 1234);
    let mut host = enet
        .create_host::<()>(
            Some(&local_addr),
            10,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )
        .context("could not create host")?;

    println!("ENet server started on localhost:1234");

    loop {
        tokio::select! {
            // Poll for Shutdown event without sleep
            Some(command) = rx.recv() => {
                match command {
                    EnetCommand::Shutdown => {
                        println!("Shutting down ENet server");
                        break;
                    }
                }
            }
            _ = async {
                tokio::time::sleep(Duration::from_millis(50)).await; // 50ms delay before checking each new request, can be made 0
            } => {
                match host.service(0).context("service failed")? {
                    Some(Event::Connect(_)) => println!("ENet: new connection!"),
                    Some(Event::Disconnect(..)) => println!("ENet: disconnect!"),
                    Some(Event::Receive {
                        channel_id,
                        ref packet,
                        ..
                    }) => println!(
                        "ENet: got packet on channel {}, content: '{}'",
                        channel_id,
                        std::str::from_utf8(packet.data()).unwrap()
                    ),
                    _ => (),
                }
            }
        }
    }

    Ok(())
}
