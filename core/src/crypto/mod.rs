pub mod ecdh;
pub mod tea;

pub struct Transport {
    s_key: [u8; 16],
    s_gt_key_new_st: [u8; 16],
    a1_key: [u8; 16],
}

impl Transport {}

pub struct StInfomation {}
