use crate::reader::Endian;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use minecraft_varint::VarIntWrite;
use std::{
    io::{Cursor, Result, Write},
    str,
};

#[derive(Clone)]
pub struct Writer {
    cursor: Cursor<Vec<u8>>,
}

impl Writer {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(buf),
        }
    }
    pub fn write(&mut self, v: &[u8]) -> Result<()> {
        self.cursor.write_all(v)
    }

    pub fn write_u8(&mut self, v: u8) -> Result<()> {
        self.cursor.write_u8(v)
    }

    pub fn write_u16(&mut self, v: u16, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u16::<BigEndian>(v),
            Endian::Little => self.cursor.write_u16::<LittleEndian>(v),
        }
    }
    pub fn write_u32(&mut self, v: u32, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u32::<BigEndian>(v),
            Endian::Little => self.cursor.write_u32::<LittleEndian>(v),
        }
    }
    pub fn write_i32(&mut self, v: i32, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_i32::<BigEndian>(v),
            Endian::Little => self.cursor.write_i32::<LittleEndian>(v),
        }
    }
    pub fn write_u24(&mut self, v: u32, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u24::<BigEndian>(v),
            Endian::Little => self.cursor.write_u24::<LittleEndian>(v),
        }
    }

    pub fn write_u64(&mut self, v: u64, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u64::<BigEndian>(v),
            Endian::Little => self.cursor.write_u64::<LittleEndian>(v),
        }
    }

    pub fn write_i64(&mut self, v: i64, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_i64::<BigEndian>(v),
            Endian::Little => self.cursor.write_i64::<LittleEndian>(v),
        }
    }

    pub fn write_vari32(&mut self, v: i32) -> Result<usize> {
        self.cursor.write_var_i32(v)
    }

    pub fn write_varu32(&mut self, v: u32) -> Result<usize> {
        self.cursor.write_var_u32(v)
    }

    pub fn write_vari64(&mut self, v: i64) -> Result<usize> {
        self.cursor.write_var_i64(v)
    }

    pub fn write_varu64(&mut self, v: u64) -> Result<usize> {
        self.cursor.write_var_u64(v)
    }

    pub fn write_string(&mut self, str: &str) -> Result<()> {
        let str_len = str.len() as u32;
        self.write_varu32(str_len)?;
        self.write(str.as_bytes())
    }

    pub fn get_raw_payload(self) -> Vec<u8> {
        self.cursor.into_inner()
    }

    pub fn pos(&self) -> u64 {
        self.cursor.position()
    }

    pub fn get_cursor(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.cursor
    }
}
