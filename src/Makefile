TARGET  ?= output

CFLAGS  += -Os -fno-common -fomit-frame-pointer -mcmodel=medany -ffreestanding -nostdinc -march=rv64gc
LDFLAGS += -static -nostdlib


.PHONY: all
all: kernel.bin kernel.elf

.PHONY: clean
clean:
	rm -f kernel.bin kernel.elf

.PHONY: install
install: kernel.bin kernel.elf
	mkdir -p $(TARGET)
	install -m 0755 $^ $(TARGET)/

kernel.elf: kernel.lds start.S kernel.cpp
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

kernel.bin: kernel.elf
	$(OBJCOPY) -O binary $< $@
