#include "asm.hpp"
#include "assert.hpp"
#include "io.hpp"
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
  // The first thread that should get a chance.
  size_t const initial_tid {clamp_tid(thread::active() - threads + 1)};

  for (size_t cur = initial_tid; cur < initial_tid + array_size(threads); cur++) {
    thread * const candidate {&threads[clamp_tid(cur)]};

    if (candidate->is_runnable()) {
      candidate->activate();
    }
  }

  format("!! XXX Implement being idle\n");
  wait_forever();
}
