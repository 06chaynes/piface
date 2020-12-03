use super::DefaultRoute;
use crate::Result;
use std::process::{Command, Output};

fn route_cmd() -> Result<Output> {
    let output = Command::new("route")
        .args(&["-n", "get", "default"])
        .output()?;
    Ok(output)
}

/// This attempts to call and parse the output of the route command
pub fn get_default_route() -> Result<DefaultRoute> {
    let mut default_route = DefaultRoute::default();
    let route_cmd_output = route_cmd()?;
    let cmd_string = String::from_utf8(route_cmd_output.stdout)?;
    for l in cmd_string.lines() {
        let sub_split = l.trim().split(':');
        let entry: Vec<&str> = sub_split.collect();
        if entry.len() == 2 {
            if entry[0] == "gateway" {
                default_route.gateway = Some(entry[1].trim_start().trim_end().to_owned())
            } else if entry[0] == "interface" {
                default_route.interface = Some(entry[1].trim_start().trim_end().to_owned())
            }
        }
    }
    Ok(default_route)
}
