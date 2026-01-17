pub fn create_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(payload.len() + 2);
    packet.push(0xAA);
    packet.extend_from_slice(payload);
    packet.push(b'\n');
    packet
}

pub fn strip_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = payload.to_vec();
    packet.remove(0);
    packet.pop();
    packet
}