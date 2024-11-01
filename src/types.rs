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
    FailedToReturnAroundNetworks,
    NetworkProfileNotFound
}