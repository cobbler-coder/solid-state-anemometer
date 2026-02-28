#pragma once
#include <zephyr/kernel.h>

// Declare that these queues exist globally
extern struct k_msgq raw_adc_queue;
extern struct k_msgq calculated_data_queue;