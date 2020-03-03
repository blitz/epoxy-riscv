        .section .text.boot
        .extern main
	.extern ctor_start_, ctor_end_

        .global asm_entry
asm_entry:
	// We come in without a stack pointer set.
        la sp, stack_end

	// Call each pointer in the constructor list.
	la s0, ctor_start_
	la s1, ctor_end_

ctor_call_loop:
	beq s0, s1, ctor_call_done

	ld a0, (s0)
	addi s0, s0, 8
	jalr a0

	j ctor_call_loop
ctor_call_done:

	// Hand over to C++ code.
        call main

        // Invoke the EXIT capability
        li a0, 0
        ecall

        .section .bss.stack
stack:
        .fill 4096
stack_end:
