#include <types.hpp>

#include "asm.hpp"
#include "assert.hpp"
#include "csr.hpp"
#include "exception_info.hpp"
#include "exception_frame.hpp"
#include "io.hpp"
#include "scheduler.hpp"
#include "thread.hpp"

namespace {

void arch_init()
{
  // Prevent executable memory from being automatically readable and disable interrupts while we are in supervisor mode.
  csr_rc<csr::SSTATUS>(SSTATUS_MXR | SSTATUS_SIE);

  // Prevent touching user memory unintentionally.
  csr_rs<csr::SSTATUS>(SSTATUS_SUM);

  // Set up exception and interrupt entry. We use direct mode where all interrupts and exceptions enter at the same
  // location.
  csr_w<csr::STVEC>(reinterpret_cast<uintptr_t>(asm_exc_entry));

  // A scratch register containing zero means we are in the kernel.
  csr_w<csr::SSCRATCH>(0);
}

[[noreturn]] void die_on_exception_from(char const *from)
{
  mword_t const sepc   {csr_r<csr::SEPC>()};
  mword_t const scause {csr_r<csr::SCAUSE>()};
  mword_t const stval  {csr_r<csr::STVAL>()};

  format("!! Exception from ", from, "!\n"
         "!! SCAUSE ", scause, "\n"
         "!! SEPC   ", sepc, "\n"
         "!! STVAL  ", stval, "\n");

  wait_forever();
}

[[noreturn]] void handle_interrupt([[maybe_unused]] exception_frame *frame, exception_info info)
{
  format("!! Unexpected interrupt: ", info.exception_code(), "\n");
  wait_forever();
}

[[noreturn]] void handle_exception(exception_frame *frame, exception_info info)
{
  assert(not info.is_interrupt());
  assert(thread::active()->frame() == frame);

  switch (info.exception_code()) {
  case exception_info::EXC_ECALL_U:
    format("!! Got system call, but it's not implemented. :(\n");
    wait_forever();
    break;
  default:
    die_on_exception_from("user");
    break;
  }
}

} // anonymous namespace

void user_exc_entry(exception_frame *frame)
{
  exception_info const info {exception_info::capture()};

  if (info.is_interrupt()) {
    handle_interrupt(frame, info);
  } else {
    handle_exception(frame, info);
  }
}

void kern_exc_entry()
{
  die_on_exception_from("kernel");
}

void start()
{
  format("\n"
         ">> Epoxy (RISC-V 64-bit, "
#ifdef __clang__
         "clang " __clang_version__
#elif __GNUC__
         "gcc " __VERSION__
#else
         "unknown compiler"
#endif
         ")\n");

  arch_init();

  schedule();
}
