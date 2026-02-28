pub mod comms;
use comms::Communication;
use anyhow::Result;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use comms::uart::UartComms;

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
use crate::comms::mock::MockComms;

pub fn poke_mcu() -> Result<()> {
    println!("Initializing hardware");
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let protocol = UartComms::new()?;
    
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let protocol = MockComms::new();

    let mut comms = Communication::new(protocol);
    comms.poke_comms()?;
    Ok(())
}

pub fn request_wind_speed() -> Result<()> {
    println!("Initializing hardware");
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let protocol = UartComms::new()?;
    
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let protocol = MockComms::new();

    let mut comms = Communication::new(protocol);
    comms.request_wind_speed()?;
    Ok(())
}