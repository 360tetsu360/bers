use std::net::SocketAddr;

use bers::{motd::Motd, server::Listener};

#[tokio::main]
async fn main() {
    let motd = Motd {
        title: "§2mcrs server!!".to_owned(),
        protocol_version: 0,
        version: "1.0".to_owned(),
        online_player: 0,
        max_player: 100,
        guid: 0,
        sub_title: "mcrs".to_owned(),
        game_mode: "Survival".to_owned(),
    };
    let local: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let mut server = Listener::new(motd, local).await;
    server.listen().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        server.recieve().await;
    }
}
