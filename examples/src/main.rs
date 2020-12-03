use piface::{PrimaryInterface, Result};

fn main() -> Result<()> {
    let iface = PrimaryInterface::load()?;
    println!("{:#?}", iface);
    Ok(())
}
