use bytes::{BufMut, Bytes, BytesMut};

pub struct Packet {
    pub seq: u32,
    pub uin: u64,
    pub packet_detail: PacketDetail,
    pub encrypt: Encrypt,
    pub command: &'static str,
    pub body: Bytes,
    pub message: String,
}

impl Packet {
    pub fn build_sso_packet(&self) -> Bytes {
        let len = self.command.len() + 20;

        let mut buf = BytesMut::with_capacity(len + self.body.len() + 4);

        let len32 = len as u32;
        buf.put_u32(len32);
        buf.put_u32(len32 - 16);

        buf.freeze()
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
