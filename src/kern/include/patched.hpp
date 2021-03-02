#pragma once

#include <epoxy-api/c_types.hpp>

// These will always be 64-bit values to simplify the implementation
// in epoxy-harden. See patched.S for their definitions.

extern "C" uint64_t const USER_SATPS[];
extern "C" uint64_t const USER_PCS[];
