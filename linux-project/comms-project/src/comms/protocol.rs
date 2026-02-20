use anyhow::Result;

/// Pads a given packet with 0xAA at the start of the packet and b'\n' at the end of the packet
pub fn create_packet(payload: &[u8], buffer: &mut [u8]) -> Result<usize> {
    let buffer_size_needed = payload.len() + 2; // 1 for header, 1 for footer
    if buffer.len() < buffer_size_needed {
        return Err(anyhow::anyhow!("Buffer size too small"));
    }
    
    buffer[0] = 0xAA; // Start Byte
    buffer[1..1+payload.len()].copy_from_slice(payload);
    buffer[1+payload.len()] = b'\n'; // TODO: Make it a checksum?
    Ok(buffer_size_needed)
}

/// Strips out the starting and ending bytes (0xAA and b'\n') of a message, returning
/// the remaining data in a vector of bytes
/// TODO: Return as a slice
pub fn strip_packet(packet: &[u8]) -> Result<&[u8]> {
    if packet.len() < 2 {
        return Err(anyhow::anyhow!("Packet too short"));
    }
    if packet[0] != 0xAA {
        return Err(anyhow::anyhow!("Invalid Header"));
    }
    // TODO: should be a checksum check
    if packet[packet.len() - 1] != b'\n' {
        return Err(anyhow::anyhow!("Invalid Footer"));
    }

    Ok(&packet[1..packet.len()-1])
}