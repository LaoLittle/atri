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
    ip_address: String,
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
}
