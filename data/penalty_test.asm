    processor 6502
    org $4020

start:
    NOP
    LDA $2030,X
    LDX #$FF
    LDA $2030,X

    JMP start


    org $FFFC
    .word start

