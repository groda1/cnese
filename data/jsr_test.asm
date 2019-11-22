    processor 6502
    org $4020

start:
    CLI
    LDA #$FF
    STA $00
    LSR
    LSR
    LSR
    LSR
    LSR
    LSR
    LSR
    LSR
    LSR $00
    LSR $00
    LSR $00
    LSR $00
    LSR $00
    LSR $00
    LSR $00
    LSR $00

loop:
    LDA #$F0
    JSR subroutine
    JMP loop

    org $8000
subroutine:
    LDX #$FF
loop1:
    STA $00,X
    DEX
    BNE loop1
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

