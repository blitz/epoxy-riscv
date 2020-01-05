        .section .text.boot
        .extern main

        .global asm_entry
asm_entry:
        la sp, stack_end
        call main
        ebreak

        .section .bss.stack
stack:
        .fill 4096
stack_end:
