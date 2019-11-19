    processor 6502
    org $4020

start:
    CLI
loop:
    INC $200,X
    JMP loop

nmisr:
    INX
    INX
    RTI
isr:
    INY
    INY
    RTI

    org $FFFA
    .word nmisr
    org $FFFC
    .word start
    org $FFFE
    .word isr

