
/* Memories definition */
MEMORY
{
  CCMSRAM    (xrw)    : ORIGIN = 0x10000000,   LENGTH = 10K
  RAM    (xrw)    : ORIGIN = 0x20000000,   LENGTH = 32K
  FLASH    (rx)    : ORIGIN = 0x08000000,   LENGTH = 128K
}
