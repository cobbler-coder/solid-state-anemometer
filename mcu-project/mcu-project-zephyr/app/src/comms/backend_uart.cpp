#include "backend_uart.hpp"

UartBackend::UartBackend(struct device* uart_device):
    m_uart_device(uart_device)
{
}