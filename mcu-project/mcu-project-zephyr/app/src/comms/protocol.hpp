#pragma once

#include <cstdint>
#include <stddef.h>

// TODO: This does not have any error recovery yet if a byte is missed
namespace protocol
{
    /// Looks for header and checksum of incoming packet, if correct header and checksum are
    /// present then strips header and checksum into raw_bytes_out and returns final packet length.
    /// Returns 0 if packet failed to process.
    /// TODO: figure out way to discard data if checksum fails (need fixed packet sizes)
    int process_incoming_bytes(const uint8_t* raw_bytes_in, size_t length_in, uint8_t* raw_bytes_out);

    /// Adds header and checksum to a packet and copies that data into packet_out. Returns final
    /// packet size.
    int build_packet(const uint8_t* raw_bytes_in, size_t length_in, uint8_t* packet_out);

    /// Calculates a checksum by XORing each successive byte and returns the calculated checksum
    uint8_t calculate_checksum(const uint8_t* payload, size_t length);
}