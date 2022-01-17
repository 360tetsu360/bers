pub mod client_to_server_handshake;
pub mod disconnect;
pub mod login_packet;
pub mod play_status;
pub mod server_to_client_handshake;
pub mod resource_pack_stack;
pub mod resource_packs_info;
use std::io::Result;

pub trait Packet: Clone {
    const ID: u8;
    fn read(buf: &[u8]) -> Result<Self>
    where
        Self: Sized;
    fn write(&self) -> Result<Vec<u8>>;
}

pub fn encode<T: Packet>(packet: T) -> Result<Vec<u8>> {
    Ok([&[T::ID], &*packet.write()?].concat())
}

pub fn decode<T: Packet>(buf: &[u8]) -> Result<T> {
    T::read(&buf[1..])
}
