pub trait Communication {
    fn send_message(&mut self, msg: &str) -> Result<(), String>;
}