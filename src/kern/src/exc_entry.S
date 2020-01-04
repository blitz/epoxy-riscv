        .extern user_exc_entry, kern_exc_entry, kern_stack_end
        .section .text.asm_exc_entry

        .global asm_exc_entry
        .align 2
asm_exc_entry:

        // TODO We actually need to save state here.

        // Exchange SSCRATCH with the stack pointer. We could use any
	// register, but sp gives short instruction encodings.
        csrrw sp, sscratch, sp

        // As we have no registers to work with, we cannot use SSTATUS
        // to figure out whether we were interrupted in the kernel. We
        // do that by checking whether sscratch was valid instead.
        //
        // We could also use a per-CPU interrupt entry that uses
	// global variables to store some registers and dispatches to the
	// right routines based on SSTATUS.
        beqz sp, asm_kernel_exc

        sd  x1,  0*8(sp)

        // x2 aka sp was clobbered and we have to get it from SSCRATCH.
        csrr x1, sscratch
        sd  x1,  1*8(sp)

        sd  x3,  2*8(sp)
        sd  x4,  3*8(sp)
        sd  x5,  4*8(sp)
        sd  x6,  5*8(sp)
        sd  x7,  6*8(sp)
        sd  x8,  7*8(sp)
        sd  x9,  8*8(sp)
        sd x10,  9*8(sp)
        sd x11, 10*8(sp)
        sd x12, 11*8(sp)
        sd x13, 12*8(sp)
        sd x14, 13*8(sp)
        sd x15, 14*8(sp)
        sd x16, 15*8(sp)
        sd x17, 16*8(sp)
        sd x18, 17*8(sp)
        sd x19, 18*8(sp)
        sd x20, 19*8(sp)
        sd x21, 20*8(sp)
        sd x22, 21*8(sp)
        sd x23, 22*8(sp)
        sd x24, 23*8(sp)
        sd x25, 24*8(sp)
        sd x26, 25*8(sp)
        sd x27, 26*8(sp)
        sd x28, 27*8(sp)
        sd x29, 28*8(sp)
        sd x30, 29*8(sp)
        sd x31, 30*8(sp)

        // Store program counter.
        csrr a0, sepc
        sd a0, 31*8(sp)

        // Store zero in SSCRATCH to indicate that we are in kernel mode.
        csrw sscratch, x0

        // The exception frame is the first parameter to user_exc_entry.
        mv a0, sp

        la sp, kern_stack_end
        j user_exc_entry

        // We end up here if we detected that we got an interrupt or
	// exception in the kernel.
asm_kernel_exc:

        // Restore our stack pointer.
        csrrw sp, sscratch, sp

        // We died in the kernel. Don't save state, but just return to C++
        // to print some diagnostics.
        j kern_exc_entry

        .section .text.asm_exc_ret
        .global asm_exc_ret
asm_exc_ret:
        mv sp, a0

        ld  x1,  0*8(sp)
        ld  x3,  2*8(sp)
        ld  x4,  3*8(sp)
        ld  x5,  4*8(sp)
        ld  x6,  5*8(sp)
        ld  x7,  6*8(sp)
        ld  x8,  7*8(sp)
        ld  x9,  8*8(sp)
        ld x10,  9*8(sp)
        ld x11, 10*8(sp)
        ld x12, 11*8(sp)
        ld x13, 12*8(sp)
        ld x14, 13*8(sp)
        ld x15, 14*8(sp)
        ld x16, 15*8(sp)
        ld x17, 16*8(sp)
        ld x18, 17*8(sp)
        ld x19, 18*8(sp)
        ld x20, 19*8(sp)
        ld x21, 20*8(sp)
        ld x22, 21*8(sp)
        ld x23, 22*8(sp)
        ld x24, 23*8(sp)
        ld x25, 24*8(sp)
        ld x26, 25*8(sp)
        ld x27, 26*8(sp)
        ld x28, 27*8(sp)
        ld x29, 28*8(sp)
        ld x30, 29*8(sp)
        ld x31, 30*8(sp)

        ld  sp,  1*8(sp)
        sret
