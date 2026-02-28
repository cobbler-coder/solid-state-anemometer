#include "protocol.hpp"

#include <string.h>

int protocol::process_incoming_bytes(const uint8_t* raw_bytes_in, size_t length_in, uint8_t* raw_bytes_out)
{
    // Must have at least header and checksum
    if(length_in < 2)
    {
        return 0;
    }    
    
    // Must have header
    if(raw_bytes_in[0] != 0xAA)
    {
        return 0;
    }

    const auto checksum = calculate_checksum(raw_bytes_in, length_in - 1);
    if(raw_bytes_in[length_in -1] != checksum)
    {
        return 0;
    }

    int processed_packet_length = length_in - 2;
    memcpy(raw_bytes_out, &raw_bytes_in[1], processed_packet_length);
    return processed_packet_length;
}

int protocol::build_packet(const uint8_t* raw_bytes_in, size_t length_in, uint8_t* packet_out)
{
    int final_packet_size = length_in + 2;
    packet_out[0] = 0xAA;
    memcpy(&packet_out[1], raw_bytes_in, length_in);

    const auto checksum = calculate_checksum(packet_out, final_packet_size - 1);
    packet_out[final_packet_size - 1] = checksum;
    return final_packet_size;
}

uint8_t protocol::calculate_checksum(const uint8_t* payload, size_t length)
{
    uint8_t checksum = 0;
    for(size_t i = 0; i < length; i++)
    {
        checksum ^= payload[i];
    }
    return checksum;
}
