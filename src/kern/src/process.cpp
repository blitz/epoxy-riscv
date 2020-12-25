#include "process.hpp"

#include "api.hpp"
#include "csr.hpp"
#include "state.hpp"

process *process::active_ = nullptr;

// This will always be 64-bit values due to limitations in
// epoxy-harden.
extern "C" uint64_t const USER_SATPS[];

kobject *process::lookup(cap_t capability)
{
  if (capability < capabilities_.length)
    return capabilities_.object[capability];

  return nullptr;
}

void process::activate()
{
  if (active_ != this) {
    active_ = this;

    csr_w<csr::SATP>(USER_SATPS[pid()]);

    // TODO We could optimize this by using ASIDs.
    asm volatile("sfence.vma" ::: "memory");
  }
}
