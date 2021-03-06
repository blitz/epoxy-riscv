#include "scheduler.hpp"

#include "asm.hpp"
#include "assert.hpp"
#include "csr.hpp"
#include "format.hpp"
#include "resources.hpp"
#include "sbi.hpp"
#include "state.hpp"
#include "util.hpp"

namespace
{
constexpr uint64_t schedule_hz {128};
constexpr uint64_t time_slice_ticks {sbitimer_freq_hz / schedule_hz};
}  // namespace

// We implement a trivial round-robin scheduling for now. This saves us from
// having a run queue or any other fancy data structure.
void schedule()
{
  while (true) {
    static_assert(array_size(threads) > 0);

    using thread_list_entry = thread *const;

    // Pointing to the last-but-one thread is mostly cosmetical to
    // ensure we schedule thread[0] initially.
    static thread_list_entry *thread_cur {&threads[array_size(threads) - 1]};
    static thread_list_entry *const thread_end {&threads[array_size(threads)]};

    for (size_t i = 0; i < array_size(threads); i++) {
      if (++thread_cur == thread_end) {
        thread_cur = &threads[0];
      }

      auto const candidate {*thread_cur};

      if (candidate->is_runnable()) {
        csr_rs<csr::SIE>(SIE_STIE);
        sbi_set_timer(rdtime() + time_slice_ticks);
        candidate->activate();
      }
    }

    format(">> We're idle.\n");

    // Enable interrupts in supervisor mode. It will be automatically
    // disabled once we get an interrupt.
    reset_stack_and_wait_for_interrupt();
  }
}
