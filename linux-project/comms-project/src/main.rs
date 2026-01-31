use comms_project::poke_mcu;
use anyhow::Result;

fn main() -> Result<()> {
    poke_mcu()?;
    Ok(())
}
