use crate::Result;
use eui48::MacAddress;
use nix::{ifaddrs::*, sys::socket::SockAddr};

/// Uses the nix crate to pull interface information
///
/// This was largely taken from https://github.com/repnop/mac_address/blob/master/src/linux.rs
pub fn get_mac(name: &str) -> Result<Option<MacAddress>> {
    let ifiter = getifaddrs()?;
    for interface in ifiter {
        if let Some(SockAddr::Link(link)) = interface.address {
            let bytes = link.addr();
            if interface.interface_name == name {
                return Ok(Some(MacAddress::from_bytes(&bytes)?));
            }
        }
    }
    Ok(None)
}
