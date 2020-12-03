use super::DefaultRoute;
use crate::Result;
use std::process::{Command, Output};

fn route_cmd() -> Result<Output> {
    let output = Command::new("wmic")
        .args(&[
            "Path",
            "Win32_IP4RouteTable",
            "Where",
            r#"Destination='0.0.0.0'"#,
            "Get",
            "InterfaceIndex,NextHop",
        ])
        .output()?;
    Ok(output)
}

fn adapter_cmd(index: &str) -> Result<Output> {
    let index_arg = format!(r#"InterfaceIndex={}"#, index);
    let output = Command::new("wmic")
        .args(&[
            "Path",
            "Win32_NetworkAdapter",
            "Where",
            &index_arg,
            "Get",
            "Name",
        ])
        .output()?;
    Ok(output)
}

/// This attempts to call and parse the wmic commands
pub fn get_default_route() -> Result<DefaultRoute> {
    let mut default_route = DefaultRoute::default();
    let route_cmd_output = route_cmd()?;
    let s = String::from_utf8(route_cmd_output.stdout)?;
    let mut lines = s.lines();
    let _titles = lines.next();
    let values = lines.next();
    if values.is_some() {
        let split = values.unwrap().split_whitespace();
        let entry: Vec<&str> = split.collect();
        if entry.len() == 2 {
            let index = entry[0];
            default_route.gateway = Some(entry[1].to_owned());
            let adapter_cmd_output = adapter_cmd(index)?;
            let sub_s = String::from_utf8(adapter_cmd_output.stdout)?;
            let mut sub_lines = sub_s.lines();
            let _sub_titles = sub_lines.next();
            let sub_values = sub_lines.next();
            if sub_values.is_some() {
                let name = sub_values.unwrap().trim_start().trim_end();
                default_route.interface = Some(name.to_owned());
            }
        }
    }
    Ok(default_route)
}
