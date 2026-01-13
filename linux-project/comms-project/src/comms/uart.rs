use super::Communication;
use rppal::uart::{Partiy, Uart, UartError};

pub struct UartComms {
    device_path: String,
    uart: Uart,
}

impl UartComms {
    pub fn new(path: Option<String>) -> Self {
        let uart_comms = Self {
            device_path: path.unwrap_or(String::from("/dev/ttyAMA0")),
        };
        uart_comms.initialize();
        uart_comms
    }

    fn initialize(self) {

    }
}

impl Communication for UartComms {

}

pub fn send_message() {
    // Do nothing
}