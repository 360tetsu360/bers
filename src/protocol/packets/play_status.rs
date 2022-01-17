use crate::{
    reader::{Endian, Reader},
    writer::Writer,
};

use super::Packet;

#[derive(Clone)]
pub enum PlayStatus {
    LoginSuccess,
    FailedClient,
    FailedServer,
    PlayerSpawn,
    FailedInvalidTenant,
    FailedVanillaEdu,
    FailedEduVanilla,
    FailedServerFull,
}

impl Packet for PlayStatus {
    const ID: u8 = 0x2;

    fn read(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = Reader::new(buf);
        let status = reader.read_i32(Endian::Big)?;
        match status {
            0 => Ok(Self::LoginSuccess),
            1 => Ok(Self::FailedServer),
            2 => Ok(Self::FailedServer),
            3 => Ok(Self::PlayerSpawn),
            4 => Ok(Self::FailedInvalidTenant),
            5 => Ok(Self::FailedVanillaEdu),
            6 => Ok(Self::FailedEduVanilla),
            7 => Ok(Self::FailedServerFull),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unknown status ID".to_owned(),
            )),
        }
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        match self {
            PlayStatus::LoginSuccess => cursor.write_i32(0, Endian::Big)?,
            PlayStatus::FailedClient => cursor.write_i32(1, Endian::Big)?,
            PlayStatus::FailedServer => cursor.write_i32(2, Endian::Big)?,
            PlayStatus::PlayerSpawn => cursor.write_i32(3, Endian::Big)?,
            PlayStatus::FailedInvalidTenant => cursor.write_i32(4, Endian::Big)?,
            PlayStatus::FailedVanillaEdu => cursor.write_i32(5, Endian::Big)?,
            PlayStatus::FailedEduVanilla => cursor.write_i32(6, Endian::Big)?,
            PlayStatus::FailedServerFull => cursor.write_i32(7, Endian::Big)?,
        }
        Ok(cursor.get_raw_payload())
    }
}
