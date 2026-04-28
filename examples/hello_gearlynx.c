#include "lynx.h"
#include "file.h"
#include "gearlynx.h"

bank0 org 0x300;
bank1 org 0x1337;
bank2 org 0x2000;

const char * hello_str_0 = "Hello gearlynx from bank 0!";
bank1 const char * hello_str_1 = "Hello gearlynx from bank 1!";
bank2 const char * hello_str_2 = "Hello gearlynx from bank 2!";

void main();

bank2 void test_bank2()
{
    gear_push_str(hello_str_2);
    gear_flush();    
    gear_push_hex(2);
    gear_flush();    
    
    main();
}


bank1 void test_bank1()
{
    gear_push_str(hello_str_1);
    gear_flush();    
    gear_push_hex(1);
    gear_flush();    

    load_bank(2);
    test_bank2();
}

void main()
{
    gear_push_str(hello_str_0);
    gear_flush();    
    gear_push_hex(0);
    gear_flush();

    load_bank(1);
    test_bank1();
}


