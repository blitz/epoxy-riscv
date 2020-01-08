#pragma once

#include <types.hpp>

// Supervisor Binary Interface v0.2
// https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc

struct sbi_retval {
  int64_t error;
  int64_t value;
};

enum class sbi_ext_id : int32_t {
  LEGACY_SET_TIMER = 0,
  LEGACY_PUTCHAR = 1,
  LEGACY_SHUTDOWN = 8,
};

enum class sbi_fun_id : int32_t {
 // Nothing here yet.
 NONE = 0,
};

inline sbi_retval sbi_ecall1(sbi_ext_id ext_id, sbi_fun_id fun_id,
			     mword_t param0)
{
  register mword_t _param0 asm ("a0") {param0};
  register mword_t _param1 asm ("a1");
  register mword_t _fun_id asm ("a6") {static_cast<mword_t>(fun_id)};
  register mword_t _ext_id asm ("a7") {static_cast<mword_t>(ext_id)};

  asm volatile ("ecall"
		: "+r" (param0), "=r" (_param1), "+r" (_fun_id), "+r" (_ext_id)
		:
		: "memory",
		  "ra", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "a2", "a3", "a4", "a5");

  // TODO Is the SBI backend guaranteed to preserve register content?

  return {static_cast<int64_t>(_param0), static_cast<int64_t>(_param1)};
}

// Well-known SBI interfaces

inline void sbi_putc(char c)
{
  sbi_ecall1(sbi_ext_id::LEGACY_PUTCHAR, sbi_fun_id::NONE, static_cast<uint8_t>(c));
}

inline void sbi_set_timer(mword_t time)
{
  sbi_ecall1(sbi_ext_id::LEGACY_SET_TIMER, sbi_fun_id::NONE, time);
}

inline void sbi_shutdown()
{
  sbi_ecall1(sbi_ext_id::LEGACY_SHUTDOWN, sbi_fun_id::NONE, 0);
}
