	processor 6502
	org $4020

begin

	sec
	sed	
	sei
	iny
	clc
	cld
	cli
	
loop
	inx
	iny
	jmp loop
    inx


