#d 0xab ; = 0xab

x = y

#if false
{
    y = 0x11
}
#elif true
{
    y = 0x22
}
#else
{
    y = 0x33
}

#d y ; = 0x22