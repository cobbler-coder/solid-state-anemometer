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
    let mut protocol = UartComms::new()
        .expect("Could not initialize UART hardware");
    
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let protocol = MockComms::new();

    let mut comms = Communication::new(protocol);
    comms.poke_comms()?;
    Ok(())
}