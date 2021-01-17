#pragma once

#include <cstddef>
#include <cstdint>

constexpr size_t fb0_height {240};
constexpr size_t fb0_width {640};

// TODO We have to hardcode the virtual addresses here for now. In
// the future epoxy-harden should generate a nice header with
// virtual addresses of shared memory regions.
inline uint16_t volatile (&fb0_pixels)[fb0_height][fb0_width] {
    *reinterpret_cast<uint16_t volatile (*)[fb0_height][fb0_width]>(0x10000000)};
