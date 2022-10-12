pub struct Packet {
    pub uin: u64,
    pub packet_detail: PacketDetail,
    pub encrypt: Encrypt,
    pub command: &'static str,
    pub body: Vec<u8>,
    pub message: String,
}

pub enum PacketDetail {
    Uin {
        seq: u32,
    },
    Login,
}

pub enum Encrypt {
    UseD2Key,
    NoEncrypt
}