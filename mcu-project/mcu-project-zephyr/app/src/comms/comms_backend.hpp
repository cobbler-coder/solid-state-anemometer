#pragma once

#include <cstdint>

class ICommsBackend
{
public:
    virtual ~ICommsBackend() = default;

    virtual void send_packet(const uint8_t* data, int length) = 0;

    virtual int read_bytes(uint8_t* buffer) = 0;
};
