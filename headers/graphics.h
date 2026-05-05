#include "lynx.h"
#include "irq.h"

#ifndef __GRAPHICS_H__
#define __GRAPHICS_H__

zp unsigned char g_swaprequested;
zp unsigned char g_drawpage;

aligned(4) const unsigned char display_buffer_0[102*80];
aligned(4) const unsigned char display_buffer_1[102*80];

void graphics_swappages() {
    if (g_drawpage) {
        *DISPADRL = display_buffer_0;
        *DISPADRH = display_buffer_0 >> 8;
        *VIDBASL = display_buffer_1;
        *VIDBASH = display_buffer_1 >> 8;
        g_drawpage = 0;
    } else {
        *DISPADRL = display_buffer_1;
        *DISPADRH = display_buffer_1 >> 8;
        *VIDBASL = display_buffer_0;
        *VIDBASH = display_buffer_0 >> 8;
        g_drawpage = 1;
    }
    g_swaprequested = 0;
}

void interrupt graphics_irq() {
    if (g_swaprequested) {
        graphics_swappages();
    }
    irq_return();
}

/// rate
///   0 -> 50 Hz
///   1 -> 60 Hz
///   2 -> 75 Hz
void graphics_init(unsigned char rate) {
    switch(rate) {
        case 0:
            X = 0xbd;
            Y = 0x31;
            break;
        case 1:
            X = 0x9e;
            Y = 0x29;
            break;
        case 2:
            X = 0x7e;
            Y = 0x20;
            break;
    }
    *HTIMBKUP = X;
    *PBKUP = Y;

    set_irq(2, graphics_irq);

    *VTIMCTLA |= 0x80;

    graphics_swappages();

    *SPRINIT = 0xF3;
}

void _graphics_draw_sprite_wait() {
    *SCBNEXTL = X;
    *SCBNEXTH = Y;
    *SPRGO = 1;
    *SDONEACK = 0;
    do {
        *CPUSLEEP = 0;
    }
    while(*SPRSYS >> 1);
    *SDONEACK = 0;
}

void _graphics_set_palette() {
    Y = 31;
    while (1) {
        GCOLMAP[Y] = libc_ptr[Y];
        Y--;
        if (!Y) {
            break;
        }
    }
}

#define graphics_draw_sprite_wait(spr_ctrl) \
    X = spr_ctrl; \
    Y = spr_ctrl >> 8; \
    _graphics_draw_sprite_wait()

#define graphics_set_palette(palette) \
    libc_ptr = palette; \
    _graphics_set_palette()

#define graphics_swap_requested() (g_swaprequested)
#define graphics_request_swap() (g_swaprequested = 1)

#endif
