use crate::reader::{Endian, Reader};

use super::Packet;

#[derive(Clone)]
pub struct LoginPacket {
    pub protocol_version: u32,
    pub chain: String,
    pub player_data: String,
}

impl Packet for LoginPacket {
    const ID: u8 = 0x1;

    fn read(buf: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Reader::new(buf);
        let protocol_version = cursor.read_u32(Endian::Big)?;
        let _data_length = cursor.read_varu32()?;
        Ok(Self {
            protocol_version,
            chain: cursor.read_string()?,
            player_data: cursor.read_string()?,
        })
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        Ok(vec![])
    }
}
