#pragma once

#include <types.hpp>

#include "csr.hpp"

class exception_info
{
  mword_t scause_;
  mword_t stval_;

  exception_info() = delete;
  exception_info(mword_t scause, mword_t stval) : scause_ {scause}, stval_ {stval} {}

public:
  enum : mword_t {
    INT_TIMER = 5,
    EXC_ECALL_U = 8,
  };

  bool is_interrupt() const { return scause_ & SCAUSE_IRQ; }

  mword_t exception_code() const { return scause_ & ~SCAUSE_IRQ; }

  static exception_info capture() { return {csr_r<csr::SCAUSE>(), csr_r<csr::STVAL>()}; }
};
