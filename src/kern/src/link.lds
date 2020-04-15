#include "memory.hpp"

OUTPUT_ARCH( "riscv" )
ENTRY(asm_unpaged_entry)

PHDRS {
      hi_rx PT_LOAD FLAGS (5); /* RX */
      hi_r  PT_LOAD FLAGS (4); /* R  */
      hi_rw PT_LOAD FLAGS (6); /* RW */
}

SECTIONS
{
  .text KERNEL_START :
  {
    *(.text.boot)
    *(.text .text.*)
  } : hi_rx

  . = ALIGN(PAGE_SIZE);
  .rodata : {
    *(.rodata .rodata.*)

    . = ALIGN(8);
    PROVIDE (ctor_start_ = .);
    *(.init_array SORT_BY_INIT_PRIORITY(.init_array.*));
    PROVIDE (ctor_end_ = .);
  } : hi_r

  . = ALIGN(PAGE_SIZE);
  .data   : { *(.data .data.*) } : hi_rw
  .bss    : { *(.bss .bss.*) } : hi_rw

  /DISCARD/ : { *(.note.GNU-stack) *(.gnu_debuglink) *(.gnu.lto_*) }
}
