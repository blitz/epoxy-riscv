TARGET  ?= output

CPPFLAGS += -Iinclude/
CFLAGS   += -flto -MD -Os -fno-common -fomit-frame-pointer -mcmodel=medlow -ffreestanding -nostdinc -march=rv64gc
CXXFLAGS += $(CFLAGS) -fno-exceptions -fno-rtti
ASFLAGS  += $(CFLAGS)
LDFLAGS  += -flto -static -nostdlib

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

kernel.elf: link.o unpaged_entry.o paged_entry.o patched.o kernel.o io.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

hello.elf: user-link.o hello.o
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ -T $^

-include *.d
