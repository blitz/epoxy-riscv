#include "asm.hpp"
#include "assert.hpp"
#include "csr.hpp"
#include "io.hpp"
#include "sbi.hpp"
#include "scheduler.hpp"
#include "state.hpp"

namespace {

size_t clamp_tid(size_t tid)
{
  return tid >= array_size(threads) ? tid - array_size(threads) : tid;
}

}

// We implement a trivial round-robin scheduling for now. This saves us from
// having a run queue or any other fancy data structure.
void schedule()
{
  while (true) {
    // The first thread that should get a chance.
    size_t const initial_tid {clamp_tid(thread::active() - threads + 1)};

    for (size_t cur = initial_tid; cur < initial_tid + array_size(threads); cur++) {
      thread * const candidate {&threads[clamp_tid(cur)]};

      if (candidate->is_runnable()) {
	// TODO Use a decent time slice length.
	sbi_set_timer(rdtime() + 1000000);
	candidate->activate();
      }
    }

    format(">> We're idle.\n");

    // Enable interrupts in supevisor mode.
    csr_rs<csr::SSTATUS>(SSTATUS_SIE);
    wait_for_interrupt();
    csr_rc<csr::SSTATUS>(SSTATUS_SIE);
  }
}
