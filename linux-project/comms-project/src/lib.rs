pub mod comms;
use comms::Communication;
use std::{thread, time};
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use comms::uart::UartComms;

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
use crate::comms::mock::MockComms;

pub fn poke_mcu() {
    println!("Initializing hardware");
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let mut comms = UartComms::new()
        .expect("Could not initialize UART hardware");
    
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let mut comms = MockComms::new();

    println!("Sending test message");
    let test_data: [u8; 5] = [0x1, 0x2, 0x3, 0x4, 0x5];
    comms.send_message(&test_data)
        .expect("UART message send failed");

    thread::sleep(time::Duration::from_millis(500));

    println!("Reading back message");
    let data_read = comms.read_message()
        .expect("UART message read failed");
    println!("Message read: {:?}", data_read);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loopback_logic() {
        let mut mock_device = MockComms::new();
        let test_payload: [u8; 5] = [0x1, 0x2, 0x3, 0x4, 0x5];

        mock_device.send_message(&test_payload).expect("Send failed");
        let received = mock_device.read_message().expect("Read failed");
        assert_eq!(received, test_payload, "Loopback failed: Data mismatch!");
    }
}