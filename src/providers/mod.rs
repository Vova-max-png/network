use crate::types::Error;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
use windows::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
use macos::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux::*;

pub trait NetworkBehavior {
    fn new() -> Self;
    fn find_around(&self) -> Result<Self, Error> where Self: Sized;
    fn get_around(&self) -> Result<&Vec<Net>, Error>;
    fn connect(&self, ssid: &str, pass: &str) -> Result<(), Error>;
}