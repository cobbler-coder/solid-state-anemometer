#include <zephyr/kernel.h>

int main(void)
{
    while(1)
    {
        printk("hello world!\n");
        k_msleep(1000);
    }
}