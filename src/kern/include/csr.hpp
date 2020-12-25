#pragma once

#include <c_types.hpp>
#include <types.hpp>

enum class csr : uint16_t {
  SSTATUS = 0x100U,
  SIE = 0x104U,
  STVEC = 0x105U,
  SSCRATCH = 0x140U,
  SEPC = 0x141U,
  SCAUSE = 0x142U,
  STVAL = 0x143U,
  SIP = 0x144U,
  SATP = 0x180U,
};

enum : mword_t {
  SSTATUS_SIE = 1UL << 1,
  SSTATUS_SPIE = 1UL << 5,
  SSTATUS_UBE = 1UL << 6,
  SSTATUS_SPP = 1UL << 8,
  SSTATUS_SUM = 1UL << 18,
  SSTATUS_MXR = 1UL << 19,

  SCAUSE_IRQ = 1UL << (__riscv_xlen - 1),

  SIE_STIE = 1UL << 5,  // Timer enable
  SIP_STIP = 1UL << 5,  // Timer pending
};

template <csr CSR>
inline void csr_w(mword_t value)
{
  asm volatile("csrw %[csr], %[val]" : : [ csr ] "i"(CSR), [ val ] "r"(value) : "memory");
}

template <csr CSR>
inline mword_t csr_r()
{
  mword_t out;

  asm volatile("csrr %[out], %[csr]" : [ out ] "=r"(out) : [ csr ] "i"(CSR));

  return out;
}

template <csr CSR>
inline mword_t csr_rc(mword_t value)
{
  mword_t out;

  asm volatile("csrrc %[out], %[csr], %[val]"
               : [ out ] "=r"(out)
               : [ csr ] "i"(CSR), [ val ] "r"(value));

  return out;
}

template <csr CSR>
inline mword_t csr_rs(mword_t value)
{
  mword_t out;

  asm volatile("csrrs %[out], %[csr], %[val]"
               : [ out ] "=r"(out)
               : [ csr ] "i"(CSR), [ val ] "r"(value));

  return out;
}
