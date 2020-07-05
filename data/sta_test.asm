    processor 6502
    org $4020

start:

    LDA #$01
    ASL
    ASL
    ASL
    ASL
    ASL
    ASL
    ASL
    ASL

    LDA #$01
    STA $46
    ASL $46
    ASL $46
    ASL $46
    ASL $46
    ASL $46
    ASL $46
    ASL $46
    ASL $46


    LDA #$FF

    DEX
    DEY
    INX
    INY

    INX
    INX

    INY

    STA $10
    STA $10,X
    STA $200
    STA $200,Y
    STA $200,X

    LDA #$30
    STA $05

    LDA #$FF
    STA ($03,X)
    STA ($05),Y

    INC $40
    INC $40,X
    INC $210
    INC $210,X
    DEC $40
    DEC $40,X
    DEC $210
    DEC $210,X

	org $FFFC
    .word start
	org $FFFE
	.word start

