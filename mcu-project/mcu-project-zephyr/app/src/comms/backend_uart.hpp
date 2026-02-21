#pragma once

#include "comms_backend.hpp"

class UartBackend : public ICommsBackend
{
public:
    UartBackend(struct device* uart_device);

    void send_packet(const uint8_t& data, int length);

    int read_bytes(uint8_t& buffer) = 0;

private:
    struct device* m_uart_device;
}