	// This code needs to be fully position independent, because
	// it is jumped to by the bootloader before paging is set up.
	//
	// We enter the paged code by triggering a page fault and
	// setting the trap vector to where we want to go.

	.section .text.boot
	.extern asm_paged_entry
	.option pic

	.global asm_unpaged_entry
asm_unpaged_entry:

	ld a0, BOOT_STVEC
	csrw stvec, a0
	
	ld a0, BOOT_SATP
	csrw satp, a0

	// We should never get here.
	sfence.vma
	wfi

	.align 3
	.global BOOT_SATP
BOOT_SATP:
	// The boot page table will be patched into the binary.
	.quad 0xFEEDFACECAFED00D

BOOT_STVEC:
	.quad asm_paged_entry
