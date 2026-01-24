#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/uart.h>
#include <zephyr/kernel.h>

#define LED0_NODE DT_ALIAS(led0)
#define COMMS_UART DT_ALIAS(comms_uart)

#define COMMS_STACK_SIZE 1024
#define COMMS_PRIORITY 7
K_THREAD_STACK_DEFINE(comms_stack_area, COMMS_STACK_SIZE);
struct k_thread comms_thread_data;

static const struct gpio_dt_spec led = GPIO_DT_SPEC_GET(LED0_NODE, gpios);

auto *external_uart = DEVICE_DT_GET(COMMS_UART);

void comms_entry_point(void* p1, void* p2, void*p3)
{
	const struct device *uart_device = reinterpret_cast<const struct device *>(p1);
	uint8_t rx_byte;

	printk("Comms Thread Started\n");
    while(1)
    {
		constexpr uint8_t tx_buf[] = {0x0, 0x1, 0x2, 0x3};
		for(const auto& value: tx_buf)
		{
			uart_poll_out(external_uart, value);
			uint8_t value_read{};
			while(uart_poll_in(external_uart, &value_read) != 0);
			printk("Read value %d\n", value_read);
			k_msleep(100);
		}
	}
}

int main(void)
{
	if (!gpio_is_ready_dt(&led))
    {
		return 0;
	}
	if(gpio_pin_configure_dt(&led, GPIO_OUTPUT_ACTIVE) < 0)
	{
		return 0;
	}

	if(!device_is_ready(external_uart))
	{
		printk("UART device not ready\n");
		return 0;
	}

	auto external_uart_ptr = &external_uart;

	k_thread_create(&comms_thread_data, comms_stack_area,
		K_THREAD_STACK_SIZEOF(comms_stack_area),
		comms_entry_point,
		(void*)(external_uart_ptr), NULL, NULL,
		COMMS_PRIORITY, 0, K_NO_WAIT);

    bool led_state = false;
    while(1)
    {
        printk("hello world!\n");
        if(gpio_pin_set_dt(&led, led_state) < 0)
		{
			printk("Error: Could not toggle pin\n");
		}
        led_state = !led_state;
        k_msleep(1000);
    }
}