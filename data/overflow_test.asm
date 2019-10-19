    processor 6502
    org $4020

    ;SEC      ; 0 - 1 = -1, returns V = 0
    LDA #$00
    SBC #$01


    LDA #$05
    SBC #$03

    LDA #$10
    SBC #$05


    LDA #$A7
    SBC #$02



    CLC      ; 1 + 1 = 2, returns C = 0
    LDA #$01
    ADC #$01

    CLC      ; 1 + -1 = 0, returns C = 1
    LDA #$01
    ADC #$FF

    CLC      ; 127 + 1 = 128, returns C = 0
    LDA #$7F
    ADC #$01

    CLC      ; -128 + -1 = -129, returns C = 1
    LDA #$80
    ADC #$FF

    CLC      ; 1 + 1 = 2, returns V = 0
    LDA #$01
    ADC #$01

    CLC      ; 1 + -1 = 0, returns V = 0
    LDA #$01
    ADC #$FF

    CLC      ; 127 + 1 = 128, returns V = 1
    LDA #$7F
    ADC #$01

    CLC      ; -128 + -1 = -129, returns V = 1
    LDA #$80
    ADC #$FF

    SEC      ; 0 - 1 = -1, returns V = 0
    LDA #$00
    SBC #$01

    SEC      ; -128 - 1 = -129, returns V = 1
    LDA #$80
    SBC #$01

    SEC      ; 127 - -1 = 128, returns V = 1
    LDA #$7F
    SBC #$FF

    SEC      ; Note: SEC, not CLC
    LDA #$3F ; 63 + 64 + 1 = 128, returns V = 1
    ADC #$40

    CLC      ; Note: CLC, not SEC
    LDA #$C0 ; -64 - 64 - 1 = -129, returns V = 1
    SBC #$40
