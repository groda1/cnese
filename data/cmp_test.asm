    processor 6502
    org $4020

start:
    CLI
loop:

    LDA #$F3
    STA $20

    LDA #$13
    LDA $20

    LDA #$01 ;  1 (signed),   1 (unsigned)
    CMP #$FF ; -1 (signed), 255 (unsigned)
    ;A = $01, C = 0, N = 0 (the subtraction result is $01 - $FF = $02), and Z = 0.

    LDA #$7F ;  127 (signed), 127 (unsigned)
    CMP #$80 ; -128 (signed), 128 (unsigned)
    A = $7F, C = 0, N = 1 (the subtraction result is $7F - $80 = $FF), and Z = 0. The comparison results are:

    LDA #$7F ;  127 (signed), 127 (unsigned)
    CMP #$7F ; -128 (signed), 128 (unsigned)


    LDA #$3F ;  127 (signed), 127 (unsigned)
    CMP #$4F ; -128 (signed), 128 (unsigned)

    LDA #$4F ;  127 (signed), 127 (unsigned)
    CMP #$3F ; -128 (signed), 128 (unsigned)

    LDA #$AA
    CMP #$22

    LDA #$17
    STA $30
    LDA #$AA
    EOR #$22
    LDA #$AA
    EOR $30

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

