TARGET  ?= output

CPPFLAGS += -Iapi/include/
CFLAGS   += -MD -Os -fno-common -fomit-frame-pointer -mcmodel=medlow -ffreestanding -nostdinc -march=rv64gc
CXXFLAGS += $(CFLAGS) -fno-exceptions -fno-rtti -std=c++17
ASFLAGS  += $(CFLAGS)
LDFLAGS  += -static -nostdlib

.PHONY: all
all: kernel.elf hello.elf

.PHONY: clean
clean:
	rm -f *.elf *.o

.PHONY: install
install: kernel.elf hello.elf
	mkdir -p $(TARGET)
	install -m 0755 $^ $(TARGET)/

# We call it .o, because that's what the .d file says regardless of what we try to name it.
%.o: %.lds
	$(CPP) $(CFLAGS) -o $@ -P -E -x c $<

kernel.elf: link.o unpaged_entry.o paged_entry.o patched.o kernel.o io.o exc_entry.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

hello.elf: user-link.o hello.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

-include *.d
