#pragma once

#include <cstdint>

namespace protocol
{
    /// Looks for header and checksum of incoming packet, if correct header and checksum are
    /// present then strips header and checksum into raw_bytes_out and returns final packet length.
    /// Returns 0 if packet failed to process.
    int process_incoming_bytes(const uint8_t* raw_bytes_in, int length_in, uint8_t* raw_bytes_out);

    /// Adds header and checksum to a packet and copies that data into packet_out. Returns final
    /// packet size.
    int build_packet(const uint8_t* raw_bytes_in, int length_in, uint8_t* packet_out);
}