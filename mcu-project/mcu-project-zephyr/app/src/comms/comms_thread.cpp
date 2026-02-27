#include "comms_thread.hpp"
#include "comms_backend.hpp"
#include "backend_uart.hpp"
#include "protocol.hpp"

#include <zephyr/kernel.h>

K_THREAD_STACK_DEFINE(comms_stack_area, 1024);
struct k_thread comms_thread_data;

void comms::comms_entry_point(void* p1, void* p2, void* p3)
{
    // TODO: put in member variable/global variable and then handle with interrupts
    // TODO: handle uart vs bluetooth
    UartBackend uart_backend = UartBackend(static_cast<const struct device*>(p1));
    auto comms_backend = &uart_backend;
    uint8_t tx_buffer[32];
    uint8_t rx_buffer[32];

    while(1)
    {
        uint8_t simulated_rx[32];
        auto rx_length = 0;
        uint8_t simulated_payload[] = {0x0, 0x1, 0x2};

        if(auto tx_message_length = protocol::build_packet(simulated_payload, sizeof(simulated_payload), tx_buffer);
            tx_message_length > 0)
        {
            comms_backend->send_packet(tx_buffer, tx_message_length);
        }

        while(protocol::process_incoming_bytes(simulated_rx, rx_length, rx_buffer) == 0);

        for(size_t i=0; i<sizeof(simulated_payload); i++)
        {
            printk("Read index %d: %d", i, rx_buffer[i]);
        }
        printk("\n");
        k_msleep(1000);
    }
}

void comms::comms_thread_init(const struct device* uart_device)
{
    k_thread_create(&comms_thread_data, comms_stack_area,
                    K_THREAD_STACK_SIZEOF(comms_stack_area),
                    comms_entry_point,
                    (void*)uart_device, NULL, NULL,
                    7, 0, K_NO_WAIT);
}