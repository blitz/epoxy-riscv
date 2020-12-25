#pragma once

// General constants
#define PAGE_SHIFT 12
#define PAGE_SIZE 4096

// The kernel starts at the top 2GB of address space
#if __riscv_xlen == 64
#define KERNEL_START 0xffffffff80000000
#elif __riscv_xlen == 32
#define KERNEL_START 0x80000000
#else
#error "Unknown architecture"
#endif
