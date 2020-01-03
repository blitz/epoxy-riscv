#pragma once

// The entry point for the C++ part of the kernel.
extern "C" [[noreturn]] void start();

// The exception/interrupt entry point in assembly.
extern "C" char asm_exc_entry[];

// The initial kernel stack pointer.
extern "C" char kern_stack_end[];

// Entrypoint for interrupts/exceptions from userspace.
extern "C" [[noreturn]] void user_exc_entry();

// Entrypoint for interrupts/exceptions from the kernel..
extern "C" [[noreturn]] void kern_exc_entry();
