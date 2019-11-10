    processor 6502
    org $4020

kek:
    LDA #$FF

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




