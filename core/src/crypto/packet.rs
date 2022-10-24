pub struct Packet {
    pub seq: u32,
    pub uin: u64,
    pub packet_detail: PacketDetail,
    pub encrypt: Encrypt,
    pub command: &'static str,
    pub body: Vec<u8>,
    pub message: String,
}

impl Packet {
    pub fn build_sso_packet(&self) {
        let len = self.command.len() + 20;

        let mut writer = BinWriter::with_capacity(len + self.body.len() + 4);
        writer.write_u32_be(len as u32);
        writer.write_u32_be((len - 16) as u32);
    }
}

pub enum PacketDetail {
    Uin,
    Login,
}

pub enum Encrypt {
    UseD2Key,
    NoEncrypt,
}

pub struct BinWriter {
    buf: Vec<u8>,
}

impl BinWriter {
    #[inline]
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.buf.extend_from_slice(slice);
    }

    #[inline]
    pub fn write_bytes(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice);
    }

    #[inline]
    pub fn write_u64_be(&mut self, val: u64) {
        self.extend_from_slice(&val.to_be_bytes());
    }

    #[inline]
    pub fn write_u32_be(&mut self, val: u32) {
        self.extend_from_slice(&val.to_be_bytes());
    }

    #[inline]
    pub fn write_u16_be(&mut self, val: u16) {
        self.extend_from_slice(&val.to_be_bytes());
    }

    #[inline]
    pub fn write_u8(&mut self, val: u8) {
        self.buf.push(val);
    }
}

impl From<Vec<u8>> for BinWriter {
    fn from(buf: Vec<u8>) -> Self {
        Self { buf }
    }
}
