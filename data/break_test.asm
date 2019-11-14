    processor 6502
    org $4020

start:
    INX
    BRK
	.byte $F0
	INX





    org $FFFC
    .word start

