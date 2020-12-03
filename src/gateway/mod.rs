//! gateway IP and interface name associated with the default route
use crate::Result;

#[cfg(target_os = "windows")]
#[path = "win.rs"]
mod os;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
#[path = "nix.rs"]
mod os;

/// Underlying function call specific to the operating system to determine the default route
/// 
/// On *nix based machines the route command will be used
/// On windows machines wmic commands will be used
pub use self::os::get_default_route;

/// This contains the gateway IP and interface name associated with the default route
/// 
/// # Examples
/// 
/// ```
/// # use piface::Result;
/// use piface::DefaultRoute;
/// 
/// # fn try_main() -> Result<()> {
/// let default_route = DefaultRoute::load()?;
/// println!("{:#?}", default_route);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct DefaultRoute {
    pub gateway: Option<String>,
    pub interface: Option<String>,
}

impl DefaultRoute {
    /// Instantiates a new DefaultRoute and calls the get method
    pub fn load() -> Result<Self> {
        let mut new = DefaultRoute::default();
        new.get()?;
        Ok(new)
    }

    /// Attempts to determine the default route
    fn get(&mut self) -> Result<()> {
        let default_route = get_default_route()?;
        self.gateway = default_route.gateway;
        self.interface = default_route.interface;
        Ok(())
    }
}

