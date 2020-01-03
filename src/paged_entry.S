        .section .text
        .extern start

        .global asm_paged_entry
asm_paged_entry:
        la sp, kern_stack_end
        j start

        .bss
        .align 12
stack:
        .fill 4096
        .global kern_stack_end
kern_stack_end:
