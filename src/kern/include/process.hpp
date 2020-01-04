#pragma once

#include "config_types.hpp"

class process {
  // Keep track of the currently activated address space.
  static process *active_;

  int pid_;
  capability_set capabilities_ {};

public:

  int pid() const { return pid_; };

  // Resolve a capability index to a kernel object pointer or nullptr, if there
  // is none.
  kobject *lookup(cap_t capability);

  // Change to this process' address space
  void activate();

  constexpr process(int pid, capability_set const &capabilities)
    : pid_ {pid}, capabilities_ {capabilities}
  { }
};
