#pragma once

// General constants
#define PAGE_SHIFT 12
#define PAGE_SIZE 4096

// The kernel starts at the top 2GB of address space
#define KERNEL_START 0xffffffff80000000

// Userspace apps start at 64K
#define USER_START 0x0000000000010000
