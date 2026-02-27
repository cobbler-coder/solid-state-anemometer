#include "backend_uart.hpp"

#include <zephyr/drivers/uart.h>

UartBackend::UartBackend(const struct device* uart_device):
    m_uart_device(uart_device)
{
}

// TODO: switch entire class to use interrupts vs polling
void UartBackend::send_packet(const uint8_t* data, int length)
{
    for(auto i=0; i<length; i++)
    {
        uart_poll_out(m_uart_device, data[i]);
    }
}

int UartBackend::read_bytes(uint8_t* buffer)
{
    uint8_t value_read{};
    int bytes_returned = 0;
    while(uart_poll_in(m_uart_device, &value_read) == 0)
    {
        buffer[bytes_returned] = value_read;
        bytes_returned++;
    }
    return bytes_returned;
}