#ifndef __GEARLYNX_H__
#define __GEARLYNX_H__

unsigned char * const GEAR_STR_LO = 0xFDC3;
unsigned char * const GEAR_STR_HI = 0xFDC4;
unsigned char * const GEAR_HEX = 0xFDC2;
unsigned char * const GEAR_FLUSH = 0xFDC0;

#define gear_push_hex(c) (*GEAR_HEX = c)

#define gear_flush() (*GEAR_FLUSH = 1)

#define gear_push_str(str_ptr) \
    *GEAR_STR_LO = str_ptr; \
    *GEAR_STR_HI = str_ptr >> 8

#endif