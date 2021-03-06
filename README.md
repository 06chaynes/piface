# `piface`

[![crates.io](https://img.shields.io/crates/v/piface.svg)](https://crates.io/crates/piface)
[![Released API docs](https://docs.rs/piface/badge.svg)](https://docs.rs/piface)

This library will attempt to determine the local machine's default route to the internet and pull related info

## Examples

```rust
use piface::{PrimaryInterface, Result};

fn main() -> Result<()> {
    let iface = PrimaryInterface::load()?;
    println!("{:#?}", iface);
    Ok(())
}
```

Outputs:

```text
PrimaryInterface {
    name: "en0",
    mac_address: MacAddress("a4:5e:60:b8:1d:2b"),
    default_route: DefaultRoute {
        gateway: Some(
            "192.168.1.1",
        ),
        interface: Some(
            "en0",
        ),
    },
}
```

## License

`piface` is licensed under both MIT and Apache 2.0
