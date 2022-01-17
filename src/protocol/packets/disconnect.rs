use crate::{reader::Reader, writer::Writer};

use super::Packet;

#[derive(Clone)]
pub struct Disconnect {
    pub hide_kick_message: bool,
    pub kick_message: String,
}

impl Packet for Disconnect {
    const ID: u8 = 0x5;

    fn read(buf: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Reader::new(buf);

        Ok(Self {
            hide_kick_message: cursor.read_u8()? != 0,
            kick_message: cursor.read_string()?,
        })
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u8(self.hide_kick_message as u8)?;
        cursor.write_string(&self.kick_message)?;
        Ok(cursor.get_raw_payload())
    }
}
