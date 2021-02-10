#include "resources.hpp"

int main()
{
  // See this code for how the GPIO is programmed:
  // https://github.com/SpinalHDL/linux/blob/spinal-5.10-latest/drivers/gpio/gpio-spinal-lib.c

  uint32_t volatile &output {gpio0_reg[1]};
  uint32_t volatile &output_enable {gpio0_reg[2]};

  output_enable = 0xFF;

  for (size_t i = 0;; i = (i + 1) % 8) {
    // We don't have a good way to sleep yet...
    for (size_t j = 0; j < 100000; j++) {
      // Turn the LED on 1/8 of the time. Otherwise, it's too bright.
      output = (((j % 8 == 0)) ? 1 : 0) << i;
    }
  }

  // Unreachable.
  return 0;
}
