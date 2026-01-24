/// Pads a given packet with 0xAA at the start of the packet and b'\n' at the end of the packet
pub fn create_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(payload.len() + 2);
    packet.push(0xAA);
    packet.extend_from_slice(payload);
    packet.push(b'\n');
    packet
}

/// Strips out the starting and ending bytes (0xAA and b'\n') of a message, returning
/// the remaining data in a vector of bytes
pub fn strip_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = payload.to_vec();
    packet.remove(0);
    packet.pop();
    packet
}