#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod uart;

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub mod mock;

pub mod protocol;
use protocol::{create_packet, strip_packet};
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::{thread, time};

pub struct Communication<T> {
    transport: T,
}

impl<T: Read + Write> Communication<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn poke_comms(&mut self) -> Result<()> {
        println!("Sending test message");
        let test_data: [u8; 5] = [0x1, 0x2, 0x3, 0x4, 0x5];
        let mut test_packet: [u8; 7] = [0; 7];
        create_packet(&test_data, &mut test_packet)?;

        self.transport.write(&test_packet)?;
        thread::sleep(time::Duration::from_millis(500));
        println!("Reading back message");
        let mut test_read_data: [u8; 7] = [0; 7];
        self.transport.read(&mut test_read_data)?;
        let read_payload = strip_packet(&test_read_data).with_context(|| "Failed to strip packet")?;
        println!("Message read: {:?}", read_payload);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::comms::mock::MockComms;

    #[test]
    fn test_loopback_logic() {
        let mock_protocol = MockComms::new();
        let mut mock_comms = Communication::new(mock_protocol);
        let test_payload: [u8; 5] = [0x1, 0x2, 0x3, 0x4, 0x5];

        mock_comms.transport.write(&test_payload).expect("Send failed");
        let mut test_read_data: [u8; 5] = [0; 5];
        mock_comms.transport.read(&mut test_read_data).expect("Read failed");
        assert_eq!(test_read_data, test_payload, "Loopback failed: Data mismatch!");
    }
}