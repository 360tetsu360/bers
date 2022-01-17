use crate::{writer::Writer, reader::{Reader, Endian}};

use super::Packet;

#[derive(Clone)]
pub struct ResourcePacksInfo {
    pub force_accept : bool,
    pub has_script : bool,
    pub force_server_packs : bool,
    pub behavior : Vec<String>,
    pub texture : Vec<String>
}

impl Packet for ResourcePacksInfo {
    const ID: u8 = 0x6;

    fn read(buf: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Reader::new(buf);
        Ok(Self { 
            force_accept : cursor.read_u8()? != 0,
            has_script : cursor.read_u8()? != 0,
            force_server_packs : cursor.read_u8()? != 0,
            behavior: vec![],
            texture: vec![],
         })
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u8(self.force_accept as u8)?;
        cursor.write_u8(self.has_script as u8)?;
        cursor.write_u8(self.force_server_packs as u8)?;
        cursor.write_varu32(4)?;
        cursor.write_u16(self.behavior.len() as u16,Endian::Big)?;
        //write behaviors
        cursor.write_u16(self.texture.len() as u16,Endian::Big)?;
        //write textures
        Ok(cursor.get_raw_payload())
    }
}


