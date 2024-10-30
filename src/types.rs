#[derive(Debug)]
pub enum NetworkType {
    Infrastructure,
    Unknown
}

#[derive(Debug)]
pub enum EncryptionType {
    Wep,
    Wpa,
    Wpa2,
    Wpa3,
    Unknown
}

#[derive(Debug)]
pub enum Error {
    GettingAroundFailed,
    FailedToReturnAroundNetworks
}

#[derive(Debug)]
pub struct Net {
    pub(crate) ssid: Box<str>,
    pub(crate) typ: NetworkType,
    pub(crate) encryption: EncryptionType,
}