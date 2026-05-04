#ifndef __IRQ_H__
#define __IRQ_H__

unsigned char * irq_jmp_table[8];
zp unsigned char irq_bkp;
zp unsigned char irq_x;

=== ASSEMBLER BEGIN ===
_irq_handler:
    phx 
    phy
    pha

    stz     irq_x
    lda     INTSET
    sta     INTRST
    sta     irq_bkp

_irq_loop:
    lda     irq_bkp
    beq     _irq_end
    lsr
    sta     irq_bkp
    bcc     _irq_return
    ldx     irq_x
    jmp     (irq_jmp_table, x)

_irq_return:
    inc     irq_x
    inc     irq_x
    bra     _irq_loop

_irq_end:
    pla
    ply
    plx
    rti

_install_irq_handler:
    sei
    lda     #$C
    sta     $FFF9
    ldx     #15
._install_irq_loop1:
    lda     #>_irq_return
    sta     irq_jmp_table, x
    dex
    lda     #<_irq_return
    sta     irq_jmp_table, x
    dex
    bpl     ._install_irq_loop1
    lda     #<_irq_handler
    sta     $FFFE
    lda     #>_irq_handler
    sta     $FFFF
    cli
    rts

_clear_irq:
    lda     #<_irq_return
    sta     irq_jmp_table, x
    lda     #>_irq_return
    sta     irq_jmp_table+1, x
    txa
    asl
    tax
    lda     TIM0CTLA, x
    and     #$7F
    sta     TIM0CTLA, x
    rts

_set_irq:
    lda     libc_tmp
    sta     irq_jmp_table, x
    lda     libc_tmp_2
    sta     irq_jmp_table+1, x
    txa
    asl
    tax
    lda     TIM0CTLA, x
    ora     #$80
    sta     TIM0CTLA, x
    rts

==== ASSEMBLER END ====

#define install_irq_handler() asm("jsr _install_irq_handler")
#define set_irq(irq, irq_func) X = (irq)*2; libc_tmp = irq_func; libc_tmp_2 = irq_func >> 8; asm("jsr _set_irq") 
#define clear_irq(irq) X = (irq)*2; asm("jsr _clear_irq")
#define irq_return() asm("jmp _irq_return")

#endif