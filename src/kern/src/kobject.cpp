#include "assert.hpp"
#include "format.hpp"
#include "kobject_all.hpp"
#include "process.hpp"
#include "sbi.hpp"
#include "scheduler.hpp"
#include "state.hpp"
#include "syscall_args.hpp"
#include "thread.hpp"
#include "util.hpp"

namespace
{
template <size_t SIZE>
void line_buffer_flush(char const *prefix, vector<char, SIZE> *line_buffer)
{
  format(prefix, " | ");
  for (char c : *line_buffer) {
    put_char(c);
  }
  put_char('\n');

  line_buffer->reset();
}

}  // namespace

syscall_result_t klog_kobject::invoke(thread *, syscall_args const &args)
{
  char const out_char = args.arg0 & 0xFF;
  bool const is_newline {out_char == '\n'};

  if (not is_newline) {
    line_buffer_.push_back(out_char);
  }

  if (is_newline or line_buffer_.capacity_left() < 1) {
    line_buffer_flush(prefix_, &line_buffer_);
  }

  return syscall_result_t::OK;
}

syscall_result_t exit_kobject::invoke(thread *thread, [[maybe_unused]] syscall_args const &args)
{
  static size_t running_threads {array_size(threads)};

  format(">> Thread of process ", thread->get_process()->pid(), " is done.\n");
  thread->exit();

  if (unlikely(--running_threads == 0)) {
    format(">> Last thread is gone. Bye bye.\n");
    sbi_shutdown();
  }

  schedule();
}
