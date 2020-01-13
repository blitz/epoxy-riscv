#include "memory.hpp"

OUTPUT_ARCH( "riscv" )
ENTRY(asm_entry)

PHDRS {
      lo_rx PT_LOAD FLAGS (5); /* RX */
      lo_r  PT_LOAD FLAGS (4); /* R  */
      lo_rw PT_LOAD FLAGS (6); /* RW */
}

SECTIONS
{
  .text USER_START :
  {
    *(.text .text.*)
  } : lo_rx

  . = ALIGN(PAGE_SIZE);
  .rodata : { *(.rodata .rodata.*) } : lo_r
  . = ALIGN(PAGE_SIZE);
  .data   : { *(.data .data.*) } : lo_rw
  .bss    : { *(.bss .bss.*) } : lo_rw

  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}
