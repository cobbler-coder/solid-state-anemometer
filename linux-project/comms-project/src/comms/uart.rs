use super::Communication;
use rppal::uart::{Parity, Uart, Error as UartError};
use std::time::Duration;
use crate::comms::protocol::{create_packet, strip_packet};

pub struct UartComms {
    connection: Uart,
}

impl UartComms {
    pub fn new() -> Result<Self, UartError> {
        // 115200 baud rate, parity: none, 8 data bits, 1 stop bit
        let mut uart = Uart::new(115200, Parity::None, 8, 1)?;

        uart.set_write_mode(true)?;

        Ok(Self {
            connection: uart,
        })
    }
}

impl Communication for UartComms {
    fn send_message(&mut self, data: &[u8]) -> Result<(), String> {
        let payload = create_packet(data);
        self.connection.write(&payload).map_err(|e| e.to_string())?;
        Ok(())
        }

    fn read_message(&mut self) -> Result<Vec<u8>, String> {
        let mut buffer = [0u8; 256];
        match self.connection.read(&mut buffer) {
            Ok(count) => {
                if count >= 2 {
                    Ok(strip_packet(&buffer[0..count]))
                } else {
                    // No data received, return empty
                    Ok(Vec::new())
                }
            }
            Err(e) => Err(format!("Uart Read Error: {}", e)),
        }
    }
}