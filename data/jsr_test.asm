    processor 6502
    org $4020

start:
    CLI


    JSR rola
    JSR rolm
    JSR rora
    JSR rorm

loop:
    LDA #$F0
    JSR subroutine
    JMP loop

    org $8000
subroutine:
    TYA
    LDX #$FF
loop1:
    STA $00,X
    DEX
    BNE loop1
    RTS

rola:
    LDA #$1
    LDX #$50
rola_loop:
    ROL
    DEX
    BNE rola_loop
    RTS

rolm:
    LDA #$1
    STA $00
    LDX #$50
rolm_loop:
    ROL $00
    DEX
    BNE rolm_loop
    RTS

rora:
    LDA #$1
    LDX #$50
rora_loop:
    ROR
    DEX
    BNE rora_loop
    RTS

rorm:
    LDA #$1
    STA $00
    LDX #$50
rorm_loop:
    ROR $00
    DEX
    BNE rorm_loop
    RTS



    org $9000
nmisr:
    INY
    RTI
isr:
    INY
    RTI

    org $FFFA
    .word nmisr
    org $FFFC
    .word start
    org $FFFE
    .word isr

