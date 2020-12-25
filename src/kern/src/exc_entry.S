        .extern user_exc_entry, kern_exc_entry, kern_stack_end
        .section .text.asm_exc_entry

#define PTR_SIZE __SIZEOF_POINTER__

#if __riscv_xlen == 64
# define STORE sd
# define LOAD ld
#elif __riscv_xlen == 32
# define STORE sw
# define LOAD lw
#else
# error Unknown platform
#endif

        .global asm_exc_entry
        .align 2
asm_exc_entry:

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

        STORE  x1,  0*PTR_SIZE(sp)

        // x2 aka sp was clobbered and we have to get it from SSCRATCH.
        csrr   x1, sscratch
        STORE  x1,  1*PTR_SIZE(sp)

        STORE  x3,  2*PTR_SIZE(sp)
        STORE  x4,  3*PTR_SIZE(sp)
        STORE  x5,  4*PTR_SIZE(sp)
        STORE  x6,  5*PTR_SIZE(sp)
        STORE  x7,  6*PTR_SIZE(sp)
        STORE  x8,  7*PTR_SIZE(sp)
        STORE  x9,  8*PTR_SIZE(sp)
        STORE x10,  9*PTR_SIZE(sp)
        STORE x11, 10*PTR_SIZE(sp)
        STORE x12, 11*PTR_SIZE(sp)
        STORE x13, 12*PTR_SIZE(sp)
        STORE x14, 13*PTR_SIZE(sp)
        STORE x15, 14*PTR_SIZE(sp)
        STORE x16, 15*PTR_SIZE(sp)
        STORE x17, 16*PTR_SIZE(sp)
        STORE x18, 17*PTR_SIZE(sp)
        STORE x19, 18*PTR_SIZE(sp)
        STORE x20, 19*PTR_SIZE(sp)
        STORE x21, 20*PTR_SIZE(sp)
        STORE x22, 21*PTR_SIZE(sp)
        STORE x23, 22*PTR_SIZE(sp)
        STORE x24, 23*PTR_SIZE(sp)
        STORE x25, 24*PTR_SIZE(sp)
        STORE x26, 25*PTR_SIZE(sp)
        STORE x27, 26*PTR_SIZE(sp)
        STORE x28, 27*PTR_SIZE(sp)
        STORE x29, 28*PTR_SIZE(sp)
        STORE x30, 29*PTR_SIZE(sp)
        STORE x31, 30*PTR_SIZE(sp)

        // Store program counter.
        csrr  a0, sepc
        STORE a0, 31*PTR_SIZE(sp)

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

        // We got an interrupt in the kernel. This can only happen,
        // because we were idle and got an interrupt or we crashed.
        // In either case, we don't need to save state here.
        j kern_exc_entry

        .section .text.asm_exc_ret
        .global asm_exc_ret
asm_exc_ret:
        mv sp, a0

        LOAD  x1,  0*PTR_SIZE(sp)
        LOAD  x3,  2*PTR_SIZE(sp)
        LOAD  x4,  3*PTR_SIZE(sp)
        LOAD  x5,  4*PTR_SIZE(sp)
        LOAD  x6,  5*PTR_SIZE(sp)
        LOAD  x7,  6*PTR_SIZE(sp)
        LOAD  x8,  7*PTR_SIZE(sp)
        LOAD  x9,  8*PTR_SIZE(sp)
        LOAD x10,  9*PTR_SIZE(sp)
        LOAD x11, 10*PTR_SIZE(sp)
        LOAD x12, 11*PTR_SIZE(sp)
        LOAD x13, 12*PTR_SIZE(sp)
        LOAD x14, 13*PTR_SIZE(sp)
        LOAD x15, 14*PTR_SIZE(sp)
        LOAD x16, 15*PTR_SIZE(sp)
        LOAD x17, 16*PTR_SIZE(sp)
        LOAD x18, 17*PTR_SIZE(sp)
        LOAD x19, 18*PTR_SIZE(sp)
        LOAD x20, 19*PTR_SIZE(sp)
        LOAD x21, 20*PTR_SIZE(sp)
        LOAD x22, 21*PTR_SIZE(sp)
        LOAD x23, 22*PTR_SIZE(sp)
        LOAD x24, 23*PTR_SIZE(sp)
        LOAD x25, 24*PTR_SIZE(sp)
        LOAD x26, 25*PTR_SIZE(sp)
        LOAD x27, 26*PTR_SIZE(sp)
        LOAD x28, 27*PTR_SIZE(sp)
        LOAD x29, 28*PTR_SIZE(sp)
        LOAD x30, 29*PTR_SIZE(sp)
        LOAD x31, 30*PTR_SIZE(sp)

        LOAD  sp,  1*PTR_SIZE(sp)
        sret
