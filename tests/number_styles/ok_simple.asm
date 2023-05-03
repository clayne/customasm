#ruledef test
{
    ld {x: u16} => $55 @ x
}

ld 0x48cf ; = 0x5548cf
ld $48cf ; = 0x5548cf
ld 0b0100100011001111 ; = 0x5548cf
ld %0100100011001111 ; = 0x5548cf
ld 0b0100_1000_1100_1111 ; = 0x5548cf
ld %0100_1000_1100_1111 ; = 0x5548cf

ld 0x1234 + 0x5678 ; = 0x5568ac
ld $1234 + $5678 ; = 0x5568ac
ld 0x1234 + $5678 ; = 0x5568ac
ld $1234 + 0x5678 ; = 0x5568ac

ld 0b0001_0010_0011_0100 + 0b0101_0110_0111_1000 ; = 0x5568ac
ld %0001_0010_0011_0100 + %0101_0110_0111_1000 ; = 0x5568ac
ld 0b0001_0010_0011_0100 + %0101_0110_0111_1000 ; = 0x5568ac
ld %0001_0010_0011_0100 + 0b0101_0110_0111_1000 ; = 0x5568ac

#d4 0b1101 % 0b1000 ; = 0x5
#d4 %1101 % %1000 ; = 0x5
#d4 %1101%%1000 ; = 0x5
#d4 %1101% %1000 ; = 0x5
#d4 %1101 % 1000 ; = 0xd

#d $abcd, $ef ; = 0xabcdef
#d $abcd`8 ; = 0xcd