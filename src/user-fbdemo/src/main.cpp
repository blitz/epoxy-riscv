#include <cstdint>
#include <cstring>
#include <cstdio>

namespace
{
// TODO We have to hardcode the virtual addresses here for now. In
// the future epoxy-harden should generate a nice header with
// virtual addresses of shared memory regions.
//
// See virtioNetAddressSpace in user-hello.dhall.
auto const framebuffer_raw {reinterpret_cast<uint16_t volatile *>(0x10000000)};
uint32_t const framebuffer_width = 640;
uint32_t const framebuffer_height = /*480*/240;
uint32_t const framebuffer_stride = 1280;

const int WIDTH = framebuffer_width;
const int HEIGHT = framebuffer_height;

const uint8_t FONT[256 * 8] = {
  #include "font.inc"
};


const uint8_t INIT_TEXT[] = {
  #include "text.inc"
};

uint8_t TEXT[HEIGHT / 8][WIDTH / 8];

void render(uint16_t volatile *pixels, int pitch, int i)
{
  for (int line = 0; line < HEIGHT; line++) {
    uint16_t volatile *pixel_line = pixels + ((line + i) % HEIGHT)*pitch;

    for (int col = 0; col < WIDTH; col++) {
      uint8_t c = TEXT[line / 8][col / 8];
      uint8_t bits = FONT[c * 8 + line % 8];
      bool on = bits & (1 << ((WIDTH - col) % 8));

      pixel_line[col] = on ? -1 : 0;
    }
  }
}

}

int main()
{
  printf("Starting font rendering loop...\n");

  memcpy(&TEXT, &INIT_TEXT, sizeof(INIT_TEXT) - 1);

  for (int i = 0; true; i++) {
    render(framebuffer_raw, framebuffer_stride / 2, i++);
  }

  return 0;
}
