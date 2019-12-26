TARGET  ?= output

CFLAGS   += -MD -Os -fno-common -fomit-frame-pointer -mcmodel=medlow -ffreestanding -nostdinc -march=rv64gc
CXXFLAGS += $(CFLAGS)
ASFLAGS  += $(CFLAGS)
LDFLAGS  += -static -nostdlib

.PHONY: all
all: kernel.bin kernel.elf hello.elf

.PHONY: clean
clean:
	rm -f kernel.bin *.elf *.o

.PHONY: install
install: kernel.bin kernel.elf hello.elf
	mkdir -p $(TARGET)
	install -m 0755 $^ $(TARGET)/

# We call it .o, because that's what the .d file says regardless of what we try to name it.
%.o: %.lds
	$(CPP) $(CFLAGS) -o $@ -P -E -x c $<

kernel.elf: link.o unpaged_entry.o paged_entry.o patched.o kernel.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

kernel.bin: kernel.elf
	$(OBJCOPY) -O binary $< $@

hello.elf: user-link.o hello.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

-include *.d
