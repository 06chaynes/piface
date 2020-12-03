use crate::Result;
use eui48::MacAddress;
use nix::{ifaddrs::*, sys::socket::SockAddr};

pub fn get_mac(name: &str) -> Result<Option<MacAddress>> {
    let ifiter = getifaddrs()?;
    for interface in ifiter {
        if let Some(address) = interface.address {
            if let SockAddr::Link(link) = address {
                let bytes = link.addr();
                if interface.interface_name == name {
                    return Ok(Some(MacAddress::from_bytes(&bytes)?));
                }
            }
        }
    }
    Ok(None)
}
