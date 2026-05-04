#include "lynx.h"
#include "irq.h"
#include "libc.h"
#include "gearlynx.h"

bank0 org 0x300;

unsigned char irq2_count;

void irq6();

void interrupt irq2() {
    gear_push_hex(2);
    gear_flush();

    irq2_count = irq2_count + 1;
    if (irq2_count > 5) {
        irq2_count = 0;
        set_irq(6, irq6);
        clear_irq(2);
    }

    irq_return();
}

void interrupt irq6() {
    gear_push_hex(6);
    gear_flush();
    
    set_irq(2, irq2);
    clear_irq(6);
    
    irq_return();
}

void main()
{
    install_irq_handler();
    
    *TIM6BKUP = 1;
    *TIM6CTLA = 0x1d;

    set_irq(2, irq2);

    while(1) {};
}
