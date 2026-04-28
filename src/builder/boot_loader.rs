use cc6502::{error::Error, generate::GeneratorState};
/*
 Karri Kaksonen, 2011

**********************************
 Here is the bootloader in plaintext
 The idea is to make the smalles possible encrypted loader as decryption
 is very slow. The minimum size is 49 bytes plus a zero byte.
**********************************
                  EXE = $fb68

                  .org $0200

                   1. force Mikey to be in memory
 9C F9 FF         stz MAPCTL

                   2. clear palette
 A0 1F            ldy #31
 A9 00            lda #0
 99 A0 FD nextc:  sta GCOLMAP, y
 88               dey
 10 FA            bpl nextc

                   3. set ComLynx to open collector
 A9 04            lda #4           a = 00000100
 8D 8C FD         sta SERCTL       set the ComLynx to open collector

                   4. set AUDIN to output
 A9 1A            lda #$1a         audin = out, rest = out,
                                   noexp = in, cart addr = out, ext pwd = in
 8D 8A FD         sta IODIR

                   5. set AUDIN to LOW
 A9 0B            lda #$0B         Set AUDIN low
 85 1A            sta $1a          Save local copy to ZP
 8D 8B FD         sta IODAT

                   6. read in_seclynxblock secondary exe + 8 bytes from the cart
                   and store it in $f000
 A2 00            ldx #0           x = 0
 A0 97            ldy #$97         y = secondary loader size (151 bytes)
 AD B2 FC rloop1: lda RCART0       read a byte from the cart
 9D 68 FB         sta EXE,X        EXE[X] = a
 E8               inx              x++
 88               dey              y--
 D0 F6            bne rloop1       loops until y wraps

                   7. jump to secondary loader
 4C 68 FB         jmp EXE
 00 00 00 00       spares
 00                End of encrypted header mark
*/
const BOOTLOADER: [u8; 52] = [
    0xff, 0xb6, 0xbb, 0x82, 0xd5, 0x9f, 0x48, 0xcf, 0x23, 0x37, 0x8e, 0x07, 0x38, 0xf5, 0xb6, 0x30,
    0xd6, 0x2f, 0x12, 0x29, 0x9f, 0x43, 0x5b, 0x2e, 0xf5, 0x66, 0x5c, 0xdb, 0x93, 0x1a, 0x78, 0x55,
    0x5e, 0xc9, 0x0d, 0x72, 0x1b, 0xe9, 0xd8, 0x4d, 0x2f, 0xe4, 0x95, 0xc0, 0x4f, 0x7f, 0x1b, 0x66,
    0x8b, 0xa7, 0xfc, 0x21,
];

const SECONDARY: &str = "
run $fb68
        ; 1. Read in the 1st File-entry (main exe) in FileEntry
        ldx #$00
        ldy #8
.rloop:
        lda $FCB2 ; RCART0      ; read a byte from the cart
        sta FileStartBlock,X ; EXE[X] = a
        inx
        dey
        bne .rloop

        ; 2. Set the block hardware to the main exe start
        lda     FileStartBlock
        sta     FileCurrBlock
        jsr     seclynxblock

        ; 3. Skip over the block offset
        lda     FileBlockOffset+1
        eor     #$FF
        tay
        lda     FileBlockOffset
        eor     #$FF
        tax
        jsr     seclynxskip0

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
        jsr     seclynxread0

        ; 5. Jump to start of the main exe code
        jmp     (FileDestAddr)

;**********************************
; Skip bytes on bank 0
; X:Y count (EOR $FFFF)
;**********************************
seclynxskip0:
        inx
        bne .skip0
        iny
        beq .block_exit
.skip0:
        jsr secreadbyte0
        bra seclynxskip0

;**********************************
; Read bytes from bank 0
; X:Y count (EOR $ffff)
;**********************************
seclynxread0:
        inx
        bne .read1
        iny
        beq .block_exit
.read1:
        jsr secreadbyte0
        sta (FileDestPtr)
        inc FileDestPtr
        bne seclynxread0
        inc FileDestPtr+1
        bra seclynxread0

;**********************************
; Read one byte from cartridge
;**********************************
secreadbyte0:
        lda $FCB2 ; RCART0
        inc FileBlockByte
        bne .block_exit
        inc FileBlockByte+1
        bne .block_exit

;**********************************
; Select a block
;**********************************
seclynxblock:
        pha
        phx
        phy
        lda __iodat
        and #$fc
        tay
        ora #2
        tax
        lda FileCurrBlock
        inc FileCurrBlock
        sec
        bra .block2
.block0:
        bcc .block1
        stx $FD8B ; IODAT
        clc
.block1:
        inx
        stx $FD87 ; SYSCTL1
        dex
.block2:
        stx $FD87 ; SYSCTL1
        rol
        sty $FD8B ; IODAT
        bne .block0
        lda __iodat
        sta $FD8B ; IODAT
        stz FileBlockByte
        lda #<($100-(>BLOCKSIZE))
        sta FileBlockByte+1
        ply
        plx
        pla

.block_exit:   rts
";

pub(crate) fn write_boot_loader(gstate: &mut GeneratorState) -> Result<(), Error> {
    let bootloaders_length = BOOTLOADER.len() + 0x97;
    gstate.write("run 0\n")?;
    let boot_loader_data = BOOTLOADER
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    gstate.write(&format!("dc.b {boot_loader_data};\n"))?;
    gstate.write(SECONDARY)?;
    gstate.write(format!("ROM_ADDR set {bootloaders_length}\n").as_str())?;
    Ok(())
}
