#include <zephyr/drivers/gpio.h>
#include <zephyr/kernel.h>

#define LED0_NODE DT_ALIAS(led0)

static const struct gpio_dt_spec led = GPIO_DT_SPEC_GET(LED0_NODE, gpios);

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