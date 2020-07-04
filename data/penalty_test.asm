    processor 6502
    org $4020

start:
    NOP
    LDA $420,X
    LDX #$FF
    LDA $420,X

    JMP start


    org $FFFC
    .word start
	org $FFFE
	.word start

