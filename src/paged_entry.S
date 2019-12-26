	.section .text
	.extern start
	
	.global asm_paged_entry
asm_paged_entry:
	la sp, stack_end
	j start

	.global sbi_putchar
sbi_putchar:
	li a6, 0
	li a7, 1
	ecall
	ret
	
	.bss
	.align 12
stack:
	.fill 4096
stack_end:
