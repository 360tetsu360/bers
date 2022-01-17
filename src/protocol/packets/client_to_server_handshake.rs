use super::Packet;

#[derive(Clone)]
pub struct Client2ServerHandshake {}

impl Packet for Client2ServerHandshake {
    const ID: u8 = 0x4;

    fn read(_ : &[u8]) -> std::io::Result<Self> {
        Ok(Self {})
    }

    fn write(&self) -> std::io::Result<Vec<u8>> {
        Ok(vec![])
    }
}
