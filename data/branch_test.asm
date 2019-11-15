    processor 6502
    org $4020


    INX
    INX
    org $4090
start:

    LDA #$10
    SEC
loop1:
    SBC #$03
    BCC out1
    JMP loop1
out1:

    LDA #$10
loop2:
    ADC #$50
    BCS out2
    JMP loop2
out2:

    LDA #$10
loop3:
    SBC #$02
    BEQ out3
    JMP loop3

out3:
    INX
    INX


    LDA #$FE
    BMI foo1
    JMP start
foo1:
    BNE foo2
    JMP start
foo2:
    LDA #$01
    BPL foo3
    JMP start
foo3:
    CLV
    BVC foo4
    JMP start
foo4:
    LDA #$50
    ADC #$70
    BVS foo5
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start
    JMP start


foo5:
    BRK
    org $FFFC
    .word start

