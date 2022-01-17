use crate::writer::Writer;

use super::Packet;

use std::{io::Error, str};

#[derive(Clone)]
pub struct Server2ClientHandshake {
    pub salt: String, //jwt
}

impl Packet for Server2ClientHandshake {
    const ID: u8 = 0x3;

    fn read(buf: &[u8]) -> std::io::Result<Self> {
        let jwt = match str::from_utf8(buf) {
            Ok(p) => p.to_owned(),
            Err(e) => return Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        };
        Ok(Self { salt: jwt })
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_string(&self.salt)?;
        Ok(cursor.get_raw_payload())
    }
}
