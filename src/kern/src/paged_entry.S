        .section .text
        .extern start

        .global asm_paged_entry
        .align 2
asm_paged_entry:

        // Give a sign of life on the console.
        li a7, 1
        li a0, 'p'
        ecall

        // We come in without a stack pointer set.
        la sp, kern_stack_end

        // Call each pointer in the constructor list.
        la s0, ctor_start_
        la s1, ctor_end_

ctor_call_loop:
        beq s0, s1, ctor_call_done

#if __riscv_xlen == 64
        ld a0, (s0)
#elif __riscv_xlen == 32
        lw a0, (s0)
#else
# error Unknown platform
#endif
        addi s0, s0, __SIZEOF_POINTER__
        jalr a0

        j ctor_call_loop
ctor_call_done:
        j start

        .bss
        .align 12
stack:
        .fill 4096
        .global kern_stack_end
kern_stack_end:
