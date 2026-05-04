#include "libc.h"

#ifndef __FILE_H__
#define __FILE_H__

=== ASSEMBLER BEGIN ===
;**********************************
; Skip bytes on bank 0
; X:Y count (EOR $FFFF)
;**********************************
_seclynxskip0:
        inx
        bne     ._skip0
        iny
        beq     ._blockexit
._skip0:
        jsr     _secreadbyte0
        bra     _seclynxskip0

;**********************************
; Read bytes from bank 0
; X:Y count (EOR $ffff)
;**********************************
_seclynxread0:
        inx
        bne     ._read1
        iny
        beq     ._blockexit
._read1:
        jsr     _secreadbyte0
        sta     (FileDestPtr)
        inc     FileDestPtr
        bne     _seclynxread0
        inc     FileDestPtr+1
        bra     _seclynxread0

;**********************************
; Read one byte from cartridge
;**********************************
_secreadbyte0:
        lda     RCART0
        inc     FileBlockByte
        bne     ._blockexit
        inc     FileBlockByte+1
        bne     ._blockexit

;**********************************
; Select a block
;**********************************
_seclynxblock:
        pha
        phx
        phy
        lda     __iodat
        and     #$fc
        tay
        ora     #2
        tax
        lda     FileCurrBlock
        inc     FileCurrBlock
        sec
        bra     ._block2
._block0:
        bcc     ._block1
        stx     IODAT
        clc
._block1:
        inx
        stx     SYSCTL1
        dex
._block2:
        stx     SYSCTL1
        rol
        sty     IODAT
        bne     ._block0
        lda     __iodat
        sta     IODAT
        stz     FileBlockByte
        lda     #<($100-(>BLOCKSIZE))
        sta     FileBlockByte+1
        ply
        plx
        pla

._blockexit:   
        rts

open_load_bank:
        stz     cctmp 
        stz     FileCurrBlock
        jsr     _seclynxblock
        ldy     #(>DIRECTORY)^$FF
        ldx     #(<DIRECTORY)^$FF
        jsr     _seclynxskip0
        lda     libc_tmp
        asl     
        rol     cctmp
        asl     
        rol     cctmp
        asl     
        rol     cctmp
        eor     #$FF
        tax
        lda     cctmp
        eor     #$FF
        tay
        jsr     _seclynxskip0

        ldx #$00
        ldy #8
.openloop:
        lda     RCART0 
        sta     FileStartBlock,X
        inx
        dey
        bne     .openloop

        lda     FileStartBlock
        sta     FileCurrBlock
        jsr     _seclynxblock

        lda     FileBlockOffset+1
        eor     #$FF
        tay
        lda     FileBlockOffset
        eor     #$FF
        tax
        jsr     _seclynxskip0

        ; 4. Read in the main exe to RAM
        lda     FileDestAddr
        ldx     FileDestAddr+1
        sta     FileDestPtr
        stx     FileDestPtr+1
        lda     FileFileLen+1
        eor     #$FF
        tay
        lda     FileFileLen
        eor     #$FF
        tax
        jmp     _seclynxread0

==== ASSEMBLER END ====

#define load_bank(x) libc_tmp = (x); asm("jsr open_load_bank");  

#endif // __FILE_H__