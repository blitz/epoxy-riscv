	.section .text.boot

	.global asm_entry
asm_entry:
	wfi
	j asm_entry
