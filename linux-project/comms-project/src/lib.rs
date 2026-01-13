pub mod comms;
use comms::Communication;
use comms::uart::UartComms;

pub fn say_hello() {
    println!("Hello, world!");
}

pub fn poke_mcu() {
    let mut comms = UartComms::new()
        .expect("Could not initialize UART hardware");

    comms.send_message("Test Message!")
        .expect("UART message send failed");
}