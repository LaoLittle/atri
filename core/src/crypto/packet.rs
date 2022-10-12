pub struct Packet {
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
        let mut sso = Vec::<u8>::with_capacity(len + self.body.len() + 4);
        sso.extend_from_slice(&len.to_be_bytes());
        sso.extend_from_slice(&(len - 16).to_be_bytes());
    }
}

pub enum PacketDetail {
    Uin { seq: u32 },
    Login,
}

pub enum Encrypt {
    UseD2Key,
    NoEncrypt,
}
