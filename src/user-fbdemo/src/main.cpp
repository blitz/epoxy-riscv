#include <cstdint>
#include <cstdio>
#include <cstring>
#include <tuple>

#include "resources.hpp"

namespace
{
const uint8_t FONT[256 * 8] = {
#include "font.inc"
};

const uint8_t INIT_TEXT[] = {
#include "text.inc"
};

uint8_t TEXT[fb0_height / 8][fb0_width / 8];

template <size_t HEIGHT, size_t STRIDE>
void render(uint16_t volatile (&pixels)[HEIGHT][STRIDE], int WIDTH, int i)
{
  for (int line = 0; line < HEIGHT; line++) {
    uint16_t volatile *pixel_line = pixels[(line + i) % HEIGHT];

    for (int col = 0; col < WIDTH; col++) {
      uint8_t c = TEXT[line / 8][col / 8];
      uint8_t bits = FONT[c * 8 + line % 8];
      bool on = bits & (1 << ((WIDTH - col) % 8));

      pixel_line[col] = on ? -1 : 0;
    }
  }
}

}  // namespace

int main()
{
  printf("Starting font rendering loop...\n");

  memcpy(&TEXT, &INIT_TEXT, sizeof(INIT_TEXT) - 1);

  for (int i = 0; true; i++) {
    render(fb0_pixels, fb0_width, i++);
  }

  return 0;
}
