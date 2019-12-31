#pragma once

// The entry point for the C++ part of the kernel.
extern "C" [[noreturn]] void start();

// The exception/interrupt entry point in assembly.
extern "C" char asm_exc_entry[];

// The C++ part of exception/interrupt entry.
extern "C" [[noreturn]] void exc_entry();
