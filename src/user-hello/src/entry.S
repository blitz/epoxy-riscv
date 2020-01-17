        .section .text.boot
        .extern main

        .global asm_entry
asm_entry:
        la sp, stack_end
        call main

        // Invoke the EXIT capability
        li a0, 0
        ecall

        .section .bss.stack
stack:
        .fill 4096
stack_end:
