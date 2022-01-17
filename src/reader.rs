use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use minecraft_varint::VarIntRead;
use std::{
    io::{Cursor, Error, Read, Result},
    str,
};

pub enum Endian {
    Big,
    Little,
}

#[derive(Clone)]
pub struct Reader<'a> {
    pub buff: &'a [u8],
    cursor: Cursor<&'a [u8]>,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buff: buf,
            cursor: Cursor::new(buf),
        }
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.cursor.read_exact(buf)?;
        Ok(())
    }
    pub fn read_u8(&mut self) -> Result<u8> {
        self.cursor.read_u8()
    }

    pub fn read_u16(&mut self, n: Endian) -> Result<u16> {
        match n {
            Endian::Big => self.cursor.read_u16::<BigEndian>(),
            Endian::Little => self.cursor.read_u16::<LittleEndian>(),
        }
    }

    pub fn read_u32(&mut self, n: Endian) -> Result<u32> {
        match n {
            Endian::Big => self.cursor.read_u32::<BigEndian>(),
            Endian::Little => self.cursor.read_u32::<LittleEndian>(),
        }
    }

    pub fn read_i32(&mut self, n: Endian) -> Result<i32> {
        match n {
            Endian::Big => self.cursor.read_i32::<BigEndian>(),
            Endian::Little => self.cursor.read_i32::<LittleEndian>(),
        }
    }

    pub fn read_u64(&mut self, n: Endian) -> Result<u64> {
        match n {
            Endian::Big => self.cursor.read_u64::<BigEndian>(),
            Endian::Little => self.cursor.read_u64::<LittleEndian>(),
        }
    }
    pub fn read_i64(&mut self, n: Endian) -> Result<i64> {
        match n {
            Endian::Big => self.cursor.read_i64::<BigEndian>(),
            Endian::Little => self.cursor.read_i64::<LittleEndian>(),
        }
    }

    pub fn read_u24(&mut self, n: Endian) -> Result<u32> {
        match n {
            Endian::Big => self.cursor.read_u24::<BigEndian>(),
            Endian::Little => self.cursor.read_u24::<LittleEndian>(),
        }
    }

    pub fn read_vari32(&mut self) -> Result<i32> {
        self.cursor.read_var_i32()
    }
    pub fn read_varu32(&mut self) -> Result<u32> {
        self.cursor.read_var_u32()
    }
    pub fn read_vari64(&mut self) -> Result<i64> {
        self.cursor.read_var_i64()
    }
    pub fn read_varu64(&mut self) -> Result<u64> {
        self.cursor.read_var_u64()
    }

    pub fn read_string(&mut self) -> Result<String> {
        let size = self.cursor.read_u32::<LittleEndian>()?;
        let str_buf = &self.buff[self.pos() as usize..(self.pos() + size as u64) as usize];
        self.next(size.into());
        match str::from_utf8(str_buf) {
            Ok(p) => Ok(p.to_owned()),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn next(&mut self, n: u64) {
        self.cursor.set_position(self.cursor.position() + n);
    }

    pub fn pos(&self) -> u64 {
        self.cursor.position()
    }

    pub fn get_cursor(&mut self) -> &mut Cursor<&'a [u8]> {
        &mut self.cursor
    }
}
