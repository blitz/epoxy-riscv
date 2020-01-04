#include "csr.hpp"
#include "state.hpp"
#include "process.hpp"

static const int num_processes = (int)array_size(processes);

process *process::active_ = nullptr;

extern "C" mword_t const USER_SATPS[num_processes];

kobject *process::lookup(cap_t capability)
{
  if (capability < capabilities_.length)
    return &kobjects[capabilities_.object[capability]];

  return nullptr;
}

void process::activate()
{
  if (active_ != this) {
    active_ = this;

    csr_w<csr::SATP>(USER_SATPS[pid()]);

    // TODO We could optimize this by using ASIDs.
    asm volatile ("sfence.vma" ::: "memory");
  }
}