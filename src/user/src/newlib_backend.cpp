#include <cstddef>
#include <machine/syscall.h>
#include <unistd.h>

#include <capabilities.hpp>

namespace {

  // The amount of heap that we can allocate via the brk() "system
  // call".
  constexpr size_t USER_HEAP_SIZE {256U << 10};

  void write_string(char const *str)
  {
    while (*str != 0) {
      invoke(CAP_KLOG, *(str++));
    }
  }
  
  ssize_t sys_write(int file, void *ptr, size_t len)
  {
    switch (file) {
    case STDOUT_FILENO:
    case STDERR_FILENO:

      for (size_t i = 0; i < len; i++) {
	invoke(CAP_KLOG, static_cast<char *>(ptr)[i]);
      }

      return len;
    default:
      return -1;
    }
  }

  long sys_close(int file)
  {
    switch (file) {
    case STDOUT_FILENO:
    case STDERR_FILENO:
      return 0;
    default:
      return -1;
    }

  }

  long sys_brk(unsigned long new_brk)
  {
    alignas(void*) static char heap[USER_HEAP_SIZE];

    uintptr_t const heap_start {reinterpret_cast<uintptr_t>(&heap[0])};
    uintptr_t const heap_end   {heap_start + sizeof(heap)};

    // Query brk
    if (new_brk == 0) {
      return heap_start;
    }

    // Set brk
    if (new_brk >= heap_start and new_brk < heap_end) {
      return new_brk;
    }

    write_string("sys_brk: Out of memory!\n");
    return -1;
  }
}

// System call backend for newlib.
//
// Our patched newlib doesn't do syscalls on its own, but calls into
// this function. We should be careful not to call back into the libc
// here to avoid problems.
extern "C" __attribute__((used))  long
__internal_syscall(long n, [[maybe_unused]] int argc,
		   long _a0, long _a1, long _a2, [[maybe_unused]] long _a3,
		   [[maybe_unused]] long _a4, [[maybe_unused]] long _a5)
{
  switch (n) {
  case SYS_write:
    return sys_write(_a0, reinterpret_cast<void *>(_a1), _a2);

  case SYS_close:
    return sys_close(_a0);

  case SYS_brk:
    return sys_brk(_a0);
    break;

  case SYS_exit:
  case SYS_exit_group:

    // Flush any remaining log messages.
    invoke(CAP_KLOG, '\n');
    invoke(CAP_EXIT);

    // We should not reach this point.
    __builtin_trap();

    break;

  case SYS_fstat:
    // TODO: Implement this to make isatty happy, which checks for st_mode & S_IFCHR.
    break;

  default:
    write_string("__internal_syscall: Unsupported system call!\n");
    break;
  }

  return -1;
}
