use crate::Result;
use eui48::MacAddress;
use std::{
    process::{Command, Output},
    str::FromStr,
};

fn index_cmd() -> Result<Output> {
    let output = Command::new("wmic")
        .args(&[
            "Path",
            "Win32_IP4RouteTable",
            "Where",
            r#"Destination='0.0.0.0'"#,
            "Get",
            "InterfaceIndex",
        ])
        .output()?;
    Ok(output)
}

fn mac_cmd(index: &str) -> Result<Output> {
    let index_arg = format!(r#"InterfaceIndex={}"#, index);
    let output = Command::new("wmic")
        .args(&[
            "Path",
            "Win32_NetworkAdapter",
            "Where",
            &index_arg,
            "Get",
            "MacAddress",
        ])
        .output()?;
    Ok(output)
}

/// Uses wmic commands to get interface information
pub fn get_mac() -> Result<Option<MacAddress>> {
    let index_cmd_output = index_cmd()?;
    let s = String::from_utf8(index_cmd_output.stdout)?;
    let mut lines = s.lines();
    let _titles = lines.next();
    let values = lines.next();
    if values.is_some() {
        let index = values.unwrap().trim_start().trim_end();
        let mac_cmd_output = mac_cmd(&index)?;
        let sub_s = String::from_utf8(mac_cmd_output.stdout)?;
        let mut sub_lines = sub_s.lines();
        let _titles = sub_lines.next();
        let sub_values = sub_lines.next();
        if sub_values.is_some() {
            let mac_str = sub_values.unwrap().trim_start().trim_end();
            let mac = MacAddress::from_str(&mac_str)?;
            return Ok(Some(mac));
        }
    }
    Ok(None)
}
