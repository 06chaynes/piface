use crate::Result;

#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod os;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
#[path = "nix.rs"]
mod os;

use self::os::get_default_route;

#[derive(Debug, Default, Clone)]
pub struct DefaultRoute {
    pub gateway: Option<String>,
    pub interface: Option<String>,
}

impl DefaultRoute {
    pub fn load() -> Result<Self> {
        let mut new = DefaultRoute::default();
        new.get()?;
        Ok(new)
    }

    fn get(&mut self) -> Result<()> {
        let default_route = get_default_route()?;
        self.gateway = default_route.gateway;
        self.interface = default_route.interface;
        Ok(())
    }
}
