use crate::{gateway::DefaultRoute, Result};
use eui48::MacAddress;
#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod os;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
#[path = "nix.rs"]
mod os;

pub use self::os::get_mac;

#[derive(Debug, Default, Clone)]
pub struct PrimaryInterface {
    pub name: String,
    pub mac_address: MacAddress,
    pub default_route: DefaultRoute,
}

impl PrimaryInterface {
    pub fn load() -> Result<Self> {
        let mut new = PrimaryInterface::default();
        new.get()?;
        Ok(new)
    }

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    fn get(&mut self) -> Result<()> {
        let gateway = DefaultRoute::load()?;
        if gateway.interface.is_some() {
            self.default_route = gateway.clone();
            self.name = gateway.interface.unwrap();
            let mac = get_mac(&self.name)?;
            if let Some(mac) = mac {
                self.mac_address = mac;
            }
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn get(&mut self) -> Result<()> {
        let gateway = DefaultRoute::load()?;
        if gateway.interface.is_some() {
            self.default_route = gateway.clone();
            self.name = gateway.interface.unwrap();
            let mac = get_mac()?;
            if let Some(mac) = mac {
                self.mac_address = mac;
            }
        }
        Ok(())
    }
}
