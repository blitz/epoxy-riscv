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

        // Give a sign of life on the console.
        li a7, 1
        li a0, 'u'
        ecall

#if __riscv_xlen == 64
        ld a0, BOOT_STVEC
        ld a1, BOOT_SATP
#elif __riscv_xlen == 32
        lw a0, BOOT_STVEC
        lw a1, BOOT_SATP
#else
# error Unknown platform
#endif
        csrw stvec, a0

        sfence.vma		// This is needed. Why?
        csrw satp, a1
        sfence.vma

        // We should never get here.
        wfi

        .align 3
        .global BOOT_SATP
BOOT_SATP:
        // The boot page table will be patched into the binary. It's always a 64-bit value for now.
        .quad 0xFEEDFACECAFED00D

BOOT_STVEC:
        .quad asm_paged_entry
