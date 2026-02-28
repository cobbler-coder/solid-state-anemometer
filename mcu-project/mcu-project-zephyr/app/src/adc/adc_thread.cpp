#include "adc_thread.hpp"
#include "backend_adc.hpp"

#include <zephyr/kernel.h>

K_THREAD_STACK_DEFINE(adc_stack_area, 1024);
struct k_thread adc_thread_data;

void adc::adc_entry_point(void* p1, void* p2, void* p3)
{
    AdcBackend adc_backend = AdcBackend(static_cast<const struct adc_dt_spec*>(p1));
    adc_backend.init();

    while(1)
    {
        auto adc_reading = adc_backend.read();
        printk("New ADC reading: %d\n", adc_reading);
        k_msleep(1000);
    }
}

void adc::adc_thread_init(const struct adc_dt_spec* adc_device)
{
    k_thread_create(&adc_thread_data, adc_stack_area,
                    K_THREAD_STACK_SIZEOF(adc_stack_area),
                    adc_entry_point,
                    (void*)adc_device, NULL, NULL,
                    7, 0, K_NO_WAIT);
}
