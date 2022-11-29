pub struct ProtocolInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub sub_version: &'static str,
    pub sign: &'static [u8; 16],
    pub build_time: u32,
    pub appid: u32,
    pub subid: u32,
    pub bitmap: u32,
    pub sigmap: u32,
    pub sso_version: u32,
    pub sdk_version: &'static str,
}

impl ProtocolInfo {
    pub const ANDROID_WATCH: ProtocolInfo = ProtocolInfo {
        id: "com.tencent.qqlite",
        name: "A2.0.5",
        version: "2.0.5",
        sub_version: "2.0.5",
        sign: &[
            166, 183, 69, 191, 36, 162, 194, 119, 82, 119, 22, 246, 243, 110, 182, 141,
        ],
        build_time: 1559564731,
        appid: 16,
        subid: 537064446,
        bitmap: 16252796,
        sigmap: 34869472,
        sso_version: 5,
        sdk_version: "6.0.0.236",
    };
}
