    processor 6502
    org $4020

start:
    LDA
    INX
    BRK
    .byte $00
    INX
    INX
    JMP start

isr:
    INY
    INY
    RTI
    org $FFFC
    .word start
    org $FFFE
    .word isr

