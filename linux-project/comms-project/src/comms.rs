#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod uart;

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub mod mock;

pub mod protocol;

pub trait Communication {
    fn send_message(&mut self, data: &[u8]) -> Result<(), String>;
    fn read_message(&mut self) -> Result<Vec<u8>, String>;
}