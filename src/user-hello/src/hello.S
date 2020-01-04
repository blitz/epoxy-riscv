        .section .text.boot

        .global asm_entry
asm_entry:
        ecall
        j asm_entry
