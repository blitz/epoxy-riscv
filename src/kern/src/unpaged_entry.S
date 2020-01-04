	// This code needs to be fully position independent, because
	// it is jumped to by the bootloader before paging is set up.
	//
	// We enter the paged code by triggering a page fault and
	// setting the trap vector to where we want to go.

	.section .text.boot
	.extern asm_paged_entry
	.option	norelax

	.global asm_unpaged_entry
asm_unpaged_entry:

	ld a0, BOOT_STVEC
	csrw stvec, a0

	ld a1, BOOT_SATP
	sfence.vma		// This is needed. Why?
	csrw satp, a1
	sfence.vma

	// We should never get here.
	wfi

	.align 3
	.global BOOT_SATP
BOOT_SATP:
	// The boot page table will be patched into the binary.
	.quad 0xFEEDFACECAFED00D

BOOT_STVEC:
	.quad asm_paged_entry
