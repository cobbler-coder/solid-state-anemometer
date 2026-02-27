#pragma once

#include "comms_backend.hpp"

#include <zephyr/device.h>
#include <zephyr/kernel.h>
#include <zephyr/sys/ring_buffer.h>

class UartBackend : public ICommsBackend
{
public:
    UartBackend(const struct device* uart_device);

    void init() override;

    void send_packet(const uint8_t* data, int length) override;

    int read_bytes(uint8_t* buffer, int max_length) override;

private:
    static void uart_isr_wrapper(const struct device* dev, void* user_data);
    void handle_isr();

    const struct device* m_uart_device;
    struct ring_buf m_rx_ringbuf;
    uint8_t m_rx_ring_buff_data[32];
    struct k_sem m_rx_sem;
};
