#include "asm.hpp"
#include "assert.hpp"
#include "csr.hpp"
#include "exception_frame.hpp"
#include "exception_info.hpp"
#include "format.hpp"
#include "process.hpp"
#include "sbi.hpp"
#include "scheduler.hpp"
#include "syscall_args.hpp"
#include "thread.hpp"

namespace {

void arch_init()
{
  // Prevent executable memory from being automatically readable and
  // disable interrupts while we are in supervisor mode.
  csr_rc<csr::SSTATUS>(SSTATUS_MXR | SSTATUS_SIE);

  // Prevent touching user memory unintentionally.
  csr_rs<csr::SSTATUS>(SSTATUS_SUM);

  // Set up exception and interrupt entry. We use direct mode where
  // all interrupts and exceptions enter at the same location.
  csr_w<csr::STVEC>(reinterpret_cast<uintptr_t>(asm_exc_entry));

  // A scratch register containing zero means we are in the kernel.
  csr_w<csr::SSCRATCH>(0);
}

[[noreturn]] void die_on_exception_from(char const *from)
{
  mword_t const sepc   {csr_r<csr::SEPC>()};
  mword_t const scause {csr_r<csr::SCAUSE>()};
  mword_t const stval  {csr_r<csr::STVAL>()};

  panic("!! Exception from ", from, "!\n"
	"!! SCAUSE ", scause, "\n"
	"!! SEPC   ", sepc, "\n"
	"!! STVAL  ", stval, "\n");
}

[[noreturn]] void handle_interrupt(exception_info info)
{
  auto const int_no {info.exception_code()};

  switch (int_no) {
  case exception_info::INT_TIMER:
    // Weirdly we have to toggle the timer enable bit to get timers to
    // do anything sane.
    csr_rc<csr::SIE>(SIE_STIE);
    schedule();
    break;
  default:
    panic("!! Unexpected interrupt: ", int_no, "\n");
    break;
  };
}

[[noreturn]] void handle_syscall(exception_frame *frame, syscall_args const &args)
{
  // We have to advance the PC manually. ECALL doesn't do that.
  frame->pc_ += 4;

  auto * const current {thread::active()};
  auto * const kobj {current->get_process()->lookup(args.cap_idx)};

  syscall_result_t res {syscall_result_t::NOCAP};

  if (likely(kobj)) {
    res = kobj->invoke(current, args);
  } else {
    format("?? Invoking invalid capability: ", args.cap_idx, "\n");
  }

  current->finish_syscall(res);
}

[[noreturn]] void handle_user_exception(exception_frame *frame, exception_info info)
{
  assert(not info.is_interrupt());
  assert(thread::active()->frame() == frame);

  switch (info.exception_code()) {
  case exception_info::EXC_ECALL_U:
    handle_syscall(frame, syscall_args {*frame});
    break;
  default:
    // TODO We shouldn't kernel panic here.
    die_on_exception_from("user");
    break;
  }
}

} // anonymous namespace

void user_exc_entry(exception_frame *frame)
{
  exception_info const info {exception_info::capture()};

  if (info.is_interrupt()) {
    handle_interrupt(info);
  } else {
    handle_user_exception(frame, info);
  }
}

void kern_exc_entry()
{
  exception_info const info {exception_info::capture()};

  if (info.is_interrupt()) {
    handle_interrupt(info);
  } else {
    die_on_exception_from("kernel");
  }
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
