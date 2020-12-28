#pragma once

#if __STDC_HOSTED__

#include <cstddef>

using mword_t = size_t;

#else

using mword_t = unsigned long;

#endif

static_assert(sizeof(mword_t) == sizeof(void *), "Machine word size is broken");
