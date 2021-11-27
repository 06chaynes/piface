//!interface name, mac address, and the related default route
use crate::{gateway::DefaultRoute, Result};
use eui48::MacAddress;
#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod os;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
#[path = "nix.rs"]
mod os;

/// Underlying function call specific to the operating system to pull a mac address for a passed interface
///
/// On *nix based machines the nix crate will be used
/// On windows machines wmic commands will be used
pub use self::os::get_mac;

/// This contains the interface name, mac address, and the related default route
///
/// The interface name will use the friendly name on windows machines
/// The mac address will be parsed for standardization
///
/// # Examples
///
/// ```
/// # use piface::Result;
/// use piface::PrimaryInterface;
///
/// # fn try_main() -> Result<()> {
/// let iface = PrimaryInterface::load()?;
/// println!("{:#?}", iface);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct PrimaryInterface {
    pub name: String,
    pub mac_address: MacAddress,
    pub default_route: DefaultRoute,
}

impl PrimaryInterface {
    /// Instantiates a new PrimaryInterface and calls the get method
    pub fn load() -> Result<Self> {
        let mut new = PrimaryInterface::default();
        new.get()?;
        Ok(new)
    }

    /// Attempts to get interface information on *nix based machines
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

    /// Attempts to get interface information on windows based machines
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
