use crate::types::{Error, Net};

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

pub trait NetworkBehavior {
    fn new() -> Self;
    fn get_around(&self) -> Result<Self, Error> where Self: Sized;
    fn print_around(&self) -> Result<&Vec<Net>, Error>;
    fn connect(&self, ssid: &str, pass: &str) -> Result<(), Error>;
}