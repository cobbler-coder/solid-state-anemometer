use std::io::{self, Read, Write};

pub struct UartComms {
    inner: Box<dyn serialport::SerialPort>,
}

impl UartComms {
    pub fn new() -> Result<Self, serialport::Error> {
        let port = serialport::new("/dev/serial0", 115_200)
            .timeout(std::time::Duration::from_millis(100))
            .open()?;
        Ok(Self{inner: port})
    }
}

impl Read for UartComms {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf) // Delegate to the inner port
    }
}

impl Write for UartComms {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf) // Delegate to the inner port
    }
    
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}