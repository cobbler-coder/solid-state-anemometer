#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod uart;

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub mod mock;

pub mod protocol;
use anyhow::Result;
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
        self.transport.write(&test_data)?;
        thread::sleep(time::Duration::from_millis(500));
        println!("Reading back message");
        let mut test_read_data: [u8; 5] = [0; 5];
        self.transport.read(&mut test_read_data)?;
        println!("Message read: {:?}", test_read_data);
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