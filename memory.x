/* Memory layout for the STM32F746 */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 320K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
