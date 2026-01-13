use super::Communication;
use rppal::uart::{Parity, Uart, Error as UartError};
use std::time::Duration;

pub struct UartComms {
    connection: Uart,
}

impl UartComms {
    pub fn new() -> Result<Self, UartError> {
        // 115200 baud rate, parity: none, 8 data bits, 1 stop bit
        let mut uart = Uart::new(115200, Parity::None, 8, 1)?;

        uart.set_read_mode(1, Duration::from_secs(1))?;
        uart.set_write_mode(true)?;

        Ok(Self {
            connection: uart,
        })
    }
}

impl Communication for UartComms {
    fn send_message(&mut self, msg: &str) -> Result<(), String> {
        let payload = format!("{}\n", msg);

        match self.connection.write(payload.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Uart Write Error: {} ", e)),
        }
    }
}