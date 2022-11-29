use crate::data::protocol::ProtocolInfo;
use crate::data::tlv::tlv16;
use bytes::{BufMut, BytesMut};
use digest::Digest;

pub struct DeviceInfo {
    display: String,
    product: String,
    device: String,
    board: String,
    brand: String,
    model: String,
    bootloader: String,
    boot_id: String,
    proc_version: String,
    base_band: String,
    sim: String,
    os_type: OSType,
    mac_address: String,
    ip_address: [u8; 4],
    wifi_bssid: String,
    wifi_ssid: String,
    imei: String,
    android_id: String,
    apn: Apn,
    version: DeviceVersion,
    imsi: [u8; 16],
}

pub enum OSType {
    Android,
    Unknown,
}

pub enum Apn {
    WiFi,
}

pub struct DeviceVersion {
    incremental: u32,
    release: String,
    codename: String,
    sdk: u8,
}

impl DeviceInfo {
    pub fn fingerprint(&self) -> String {
        format!(
            "{}/{}/{}:10/{}/{}:user/release-keys",
            self.brand, self.product, self.device, self.android_id, self.version.incremental
        )
    }

    pub fn guid(&self) -> [u8; 16] {
        let mut md5 = md5::Md5::default();
        md5.update(&self.imei);
        md5.update(&self.mac_address);

        let mut arr = [0; 16];
        arr.copy_from_slice(&md5.finalize());
        arr
    }
}

#[test]
fn a() {
    let device = DeviceInfo {
        display: "RICQ.448911.001".into(),
        product: "iarim".into(),
        device: "sagit".into(),
        board: "eomam".into(),
        model: "MI 6".into(),
        boot_id: "b15f6fbe-5e29-7d9b-9e73-6a400fa19f08".into(),
        proc_version: "Linux 5.4.0-54-generic-QXp0SDMw (android-build@google.com)".into(),
        imei: "582827438036112".into(),
        brand: "Xiaomi".into(),
        bootloader: "U-boot".into(),
        base_band: "".into(),
        version: DeviceVersion {
            incremental: 5891938,
            release: "10".into(),
            codename: "REL".into(),
            sdk: 29,
        },
        sim: "T-Mobile".into(),
        os_type: OSType::Android,
        mac_address: "00:50:56:C0:00:08".into(),
        ip_address: [10, 0, 1, 3],
        wifi_bssid: "00:50:56:C0:00:08".into(),
        wifi_ssid: "MiWifi".into(),
        imsi: [
            6, 202, 21, 94, 8, 98, 96, 116, 162, 56, 14, 239, 3, 169, 240, 149,
        ],
        android_id: "d7fc70a09f4cc4f5".into(),
        apn: Apn::WiFi,
    };

    let protocol = ProtocolInfo::ANDROID_WATCH;
    let guid = device.guid();

    let mut b = BytesMut::new();
    b.put_u16(0);
    b.put_u32(16);
    b.put_u64(0);
    b.put_u8(8);
    b.put_u16(0);
    b.put_u16(6);
    b.put_slice(&tlv16(&protocol, &guid));

    println!("{:?}", &*b);
}
