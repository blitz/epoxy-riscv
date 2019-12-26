#pragma once

#include <types.hpp>

enum class csr : uint16_t {
  STVEC = 0x105U,
  SATP  = 0x180U,
};

template <csr CSR>
inline void write_csr(mword_t value)
{
  asm volatile ("csrw %[csr], %[val]"
		:
		: [csr] "i" (CSR), [val] "r" (value)
		: "memory");
}
