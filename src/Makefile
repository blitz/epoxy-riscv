TARGET   ?= output

USER_BINARIES := hello.user.elf
BINARIES      := $(USER_BINARIES) epoxy.kern.elf

MACHINE_DESC     ?= examples/riscv-qemu-machine.json
APPLICATION_DESC ?= examples/user-hello.json

.PHONY: all
all: $(BINARIES)

.PHONY: clean
clean:
	rm -f *.elf kern/src/*.[do] user-hello/src/*.[do]
	rm -f kern/include/state.hpp kern/src/state.cpp

.PHONY: install
install: $(BINARIES)
	mkdir -p $(TARGET)
	install -m 0755 $^ $(TARGET)/

COMMON_CFLAGS   = -flto -Iapi/include/ -MD -Os -fno-common -fomit-frame-pointer -mcmodel=medlow -ffreestanding -nostdinc -march=rv64gc
COMMON_CXXFLAGS = $(COMMON_CFLAGS) -fno-exceptions -fno-rtti -std=c++17
COMMON_ASFLAGS  = $(COMMON_CFLAGS)
COMMON_LDFLAGS  = -flto -static -nostdlib

KERN_CFLAGS := $(COMMON_CFLAGS)
USER_CFLAGS := $(COMMON_CFLAGS)

KERN_CXXFLAGS := $(COMMON_CXXFLAGS)
USER_CXXFLAGS := $(COMMON_CXXFLAGS)

KERN_LDFLAGS := $(COMMON_LDFLAGS)
USER_LDFLAGS := $(COMMON_LDFLAGS)

# We call it .o, because that makes the file name transformations easier below.
%.kern.o: %.lds
	$(CPP) $(KERN_CFLAGS) -MT $@ -c -o $@ -P -E -x c $<

%.kern.o: %.S
	$(CC) $(KERN_CFLAGS) -c -o $@ $<

%.kern.o: %.cpp
	$(CC) $(KERN_CXXFLAGS) -c -o $@ $<

%.user.o: %.lds
	$(CPP) $(USER_CFLAGS) -c -o $@ -P -E -x c $<

%.user.o: %.S
	$(CC) $(USER_CFLAGS) -c -o $@ $<

%.user.o: %.cpp
	$(CC) $(USER_CXXFLAGS) -c -o $@ $<

%.kern.elf:
	$(CC) $(KERN_CFLAGS) $(KERN_LDFLAGS) -o $@ -T $^

%.user.elf:
	$(CC) $(USER_CFLAGS) $(USER_LDFLAGS) -o $@ -T $^

# We need to jump through this hoop, because Make is broken in the way
# it handles multiple targets in normal rules. Pattern rules don't
# have this problem.
kern/src/%.cpp kern/include/%.hpp: kern/src/%.dummy $(MACHINE_DESC) $(APPLICATION_DESC) $(USER_BINARIES)
	./scripts/config --machine $(MACHINE_DESC) --application $(APPLICATION_DESC) \
                         --output-cpp kern/src/state.cpp \
                         --output-hpp kern/include/state.hpp

kern/src/state.dummy:
	touch $@

KERN_OBJ := $(addsuffix .kern.o, $(addprefix kern/src/, \
    link unpaged_entry paged_entry patched process kernel io exc_entry thread state))
USER_OBJ := $(addsuffix .user.o, $(addprefix user-hello/src/, link hello))

$(KERN_OBJ): kern/include/state.hpp

epoxy.kern.elf: KERN_CXXFLAGS += -Ikern/include
epoxy.kern.elf: $(KERN_OBJ)

hello.user.elf: $(USER_OBJ)
hello.user.elf: USER_CFLAGS += -Iuser-hello/include

qemu-example-hello.elf: $(BINARIES) examples/user-hello.json examples/user-hello.json examples/riscv-qemu-machine.json
	epoxy-harden --machine $(MACHINE_DESC) \
                     --application $(APPLICATION_DESC) \
                     --kernel epoxy.kern.elf --output $@

.PHONY: qemu-boot-hello
qemu-boot-hello: qemu-example-hello.elf
	boot -display none -device loader,file=$<

-include kern/src/*.d
-include user-hello/src/*.d
