use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};

use aes_ctr::cipher::{
    generic_array::GenericArray,
    stream::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek},
};
use aes_ctr::Aes256Ctr;

use ring::digest;

use super::error::CryptErr;

pub struct Cipher {
    secret: Vec<u8>,
    cipher: Aes256Ctr,
    decipher: Aes256Ctr,
    receive: u64,
    send: u64,
}

impl Cipher {
    pub fn new(skey: &[u8]) -> Result<Self, CryptErr> {
        let mut a = "".to_string();
        for i in skey {
            a += &format!("{:02x} ",i);
        }
        dbg!(a);
        let key = GenericArray::from_slice(skey);
        let nonce: &[u8] = &[&skey[..12], &[0, 0, 0, 2]].concat();
        let nonce_array = GenericArray::from_slice(nonce);
        let cipher = Aes256Ctr::new(key, nonce_array);
        let mut decipher = Aes256Ctr::new(key, nonce_array);
        decipher.seek(0);
        Ok(Self {
            secret: skey.to_vec(),
            cipher,
            decipher,
            receive: 0,
            send: 0,
        })
    }

    pub fn check_sum(&mut self, payload: &[u8]) -> Result<(), CryptErr> {
        let payload_len = payload.len();
        let data = &payload[..payload_len - 8];
        let sum = &payload[payload_len - 8..payload_len];

        let digest_alg = &digest::SHA256;

        let le_bytes = match get_le(self.receive) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::Other(e.to_string())),
        };
        let mut digest = digest::Context::new(digest_alg);
        digest.update(&le_bytes);
        digest.update(data);
        digest.update(&self.secret);

        let digest_bytes = digest.finish();

        let peer_sum = &digest_bytes.as_ref()[..8];
        if !peer_sum.eq(sum) {
            return Err(CryptErr::BadPacket);
        }

        self.receive += 1;

        Ok(())
    }

    pub fn write_sum(&mut self, payload: &mut Vec<u8>) -> Result<(), CryptErr> {
        let digest_alg = &digest::SHA256;

        let le_bytes = match get_le(self.send) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::Other(e.to_string())),
        };
        let mut digest = digest::Context::new(digest_alg);
        digest.update(&le_bytes);
        digest.update(payload);
        digest.update(&self.secret);

        let mut digest_bytes = digest.finish().as_ref()[..8].to_vec();
        payload.append(&mut digest_bytes);

        Ok(())
    }

    pub fn decrypt(&mut self, payload: &mut Vec<u8>) {
        self.decipher.apply_keystream(payload);
    }

    pub fn encrypt(&mut self, payload: &mut Vec<u8>) {
        self.cipher.apply_keystream(payload);
    }

    pub fn decode(&mut self, payload: &mut Vec<u8>) -> Result<(), CryptErr> {
        self.decrypt(payload);
        self.check_sum(payload)?;
        Ok(())
    }

    pub fn encode(&mut self, payload: &mut Vec<u8>) -> Result<(), CryptErr> {
        self.write_sum(payload)?;
        self.encrypt(payload);
        Ok(())
    }
}

fn get_le(v: u64) -> std::io::Result<Vec<u8>> {
    let mut cursor = Cursor::new(vec![]);
    cursor.write_u64::<LittleEndian>(v)?;
    Ok(cursor.into_inner())
}
