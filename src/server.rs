use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use raknet::{RaknetEvent, Server};
use tokio::sync::Mutex;

use crate::{connection::Connection, motd::Motd};
pub struct Listener {
    socket: Arc<Mutex<Server>>,
    connections: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
}

impl Listener {
    pub async fn new(mut motd: Motd, address: SocketAddr) -> Self {
        let ret = Self {
            socket: Arc::new(Mutex::new(Server::new(address, "".to_owned()))),
            connections: Arc::new(Mutex::new(HashMap::new())),
        };
        motd.guid = ret.socket.lock().await.id;
        ret.socket
            .lock()
            .await
            .set_motd(motd.to_string())
            .await
            .unwrap();
        ret
    }
    pub async fn listen(&mut self) {
        self.socket.lock().await.listen().await.unwrap();
        let socket = self.socket.clone();
        let connections = self.connections.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                let events = socket.lock().await.recv().await.unwrap();
                for event in events {
                    match event {
                        RaknetEvent::Packet(p) => {
                            connections
                                .lock()
                                .await
                                .get_mut(&p.address)
                                .unwrap()
                                .handle(p);
                        }
                        RaknetEvent::Connected(s, i) => {
                            dbg!(i);
                            connections
                                .lock()
                                .await
                                .insert(s, Connection::new(socket.clone(), s));
                        }
                        RaknetEvent::Disconnected(s, _i, _r) => {
                            connections.lock().await.get_mut(&s).unwrap().disconnected();
                            connections.lock().await.remove(&s);
                        }
                        RaknetEvent::Error(s, e) => {
                            connections.lock().await.remove(&s);
                            eprintln!("Raknet Error : {}", e);
                        }
                    }
                }
                for conn in connections.lock().await.values_mut() {
                    conn.update().await;
                }
            }
        });
    }
    pub async fn recieve(&mut self) {
    }
}
