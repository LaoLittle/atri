use prost::{Enumeration, Message};

#[derive(Message)]
pub struct MessageHead {
    #[prost(uint64)]
    from_uin: u64,
    #[prost(uint64)]
    to_uin: u64,
    #[prost(enumeration = "MessageHeadType")]
    msg_type: i32,
}

#[derive(Debug, Enumeration)]
#[repr(i32)]
pub enum MessageHeadType {
    Troop = 82,
}
