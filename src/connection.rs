use std::{
    io::{Cursor, Read, Write},
    net::SocketAddr,
    sync::Arc,
};

use flate2::read::DeflateDecoder;
use minecraft_varint::VarIntRead;
use raknet::{packet::RaknetPacket, Server};
use tokio::sync::Mutex;

use crate::{
    protocol::{
        crypto::cipher::Cipher,
        login::{
            exchange::exchange,
            verify::{self, verify_skin},
        },
        packets::{
            client_to_server_handshake::Client2ServerHandshake, decode, encode,
            login_packet::LoginPacket, server_to_client_handshake::Server2ClientHandshake, Packet, play_status::PlayStatus, resource_packs_info::ResourcePacksInfo,
        },
    },
    writer::Writer,
};

const PROTOCOL_VERSION : u32 = 475;

fn get_varint(value: u32) -> std::io::Result<Vec<u8>> {
    let mut cursor = Writer::new(vec![]);
    cursor.write_varu32(value)?;
    Ok(cursor.get_raw_payload())
}

fn frame<T: Packet>(p: T) -> std::io::Result<Vec<u8>> {
    let buff = encode::<T>(p)?;
    let payload = get_varint(buff.len() as u32)?;
    Ok([&payload, &*buff].concat())
}

pub struct Connection {
    socket: Arc<Mutex<Server>>,
    address: SocketAddr,
    send_queue: Vec<u8>,
    encryption : bool,
    cipher: Option<Cipher>,
}

impl Connection {
    pub fn new(socket: Arc<Mutex<Server>>, address: SocketAddr) -> Self {
        Self {
            socket,
            address,
            send_queue: vec![],
            encryption : false,
            cipher: None,
        }
    }
    pub fn handle(&mut self, mut packet: RaknetPacket) {
        if packet.data.remove(0) != 0xfe {
            return;
        }

        if self.cipher.is_some() {
            match self.cipher.as_mut().unwrap().decode(&mut packet.data) {
                Ok(_) => {}
                Err(e) => {
                    if let crate::protocol::crypto::error::CryptErr::BadPacket = e {
                        self.bad_packet();
                    }
                }
            };
        }

        let mut decompressor = DeflateDecoder::new(&*packet.data);

        // deflate packet
        let mut data = vec![];

        match decompressor.read_to_end(&mut data) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("decompressing error {}", e);
                return;
            }
        };

        let mut cursor = Cursor::new(&data);

        // split packet
        let mut packets: Vec<&[u8]> = vec![];
        while cursor.position() < data.len() as u64 {
            let length = match cursor.read_var_u32() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };

            if cursor.position() + length as u64 > data.len() as u64 {
                break;
            }

            let packet =
                &data[cursor.position() as usize..(cursor.position() + length as u64) as usize];
            packets.push(packet);

            cursor.set_position(cursor.position() + length as u64);
        }

        for packet in packets {
            self.handle_packet(packet);
        }
    }
    pub fn handle_packet(&mut self, payload: &[u8]) {
        // decode packet
        match payload[0] {
            LoginPacket::ID => {
                self.handle_login(payload);
            }
            Client2ServerHandshake::ID => {
                self.encryption = true;

                let play_satus = PlayStatus::LoginSuccess;

                self.send(play_satus).unwrap();
            }
            0x81 => {
                let resource_info = ResourcePacksInfo{ force_accept: false, has_script: false, force_server_packs : false,behavior: vec![], texture: vec![] };
                self.send(resource_info).unwrap();
            }
            _ => {
                println!("unknown packet ID {}",&payload[0]);
            }
        }
    }

    pub fn handle_login(&mut self, payload: &[u8]) {
        let login = match decode::<LoginPacket>(payload) {
            Ok(p) => p,
            Err(e) => {
                dbg!(e);
                self.disconnected();
                return;
            }
        };

        //dbg!(&login.chain);

        if login.protocol_version != PROTOCOL_VERSION {
            let play_satus = PlayStatus::FailedClient;
            self.send(play_satus).unwrap();
        }

        let verify = match verify::verify(login.chain) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("verify jwt error {}", e);
                self.disconnected();
                return;
            }
        };
        let pubkey = verify.0;
        let extra_data = verify.1;

        println!("connected {} {}", extra_data.display_name, extra_data.xuid);

        let _player_data = match verify_skin(login.player_data, &pubkey) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("verify player data error {}", e);
                self.disconnected();
                return;
            }
        };

        let (jwt, cipher) = match exchange(pubkey) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("key exchange error {}", e);
                self.disconnected();
                return;
            }
        };

        match self.send(Server2ClientHandshake { salt: jwt }) {
            Ok(p) => p,
            Err(e) => eprintln!("error while encoding server2client {}", e),
        }

        self.cipher = Some(cipher);
    }

    pub fn send<T: Packet>(&mut self, packet: T) -> std::io::Result<()> {
        let mut buff = frame(packet)?;
        self.send_queue.append(&mut buff);
        Ok(())
    }

    pub async fn update(&mut self) {
        if !self.send_queue.is_empty() {
            let mut compressor =
                flate2::write::DeflateEncoder::new(vec![], flate2::Compression::new(7));
            compressor.write_all(&self.send_queue).unwrap();
            let mut compressed = compressor.finish().unwrap();

            if self.encryption {
                self.cipher
                    .as_mut()
                    .unwrap()
                    .encode(&mut compressed)
                    .unwrap(); //とりあえずunwrap
            }

            compressed.insert(0, 0xFE); //MCPE Packet
            self.socket
                .lock()
                .await
                .send_to(&self.address, &compressed)
                .await
                .unwrap();
            self.send_queue.clear();
        }
    }
    pub fn bad_packet(&mut self) {
        todo!()
    }
    /*pub fn disconnect(&mut self) {
        let disconnect = Disconnect{hide_kick_message:false, kick_message: "something went wrong :(".to_owned() };

        self.send(disconnect).unwrap();
    }*/
    pub fn disconnected(&mut self) {
    }
}
