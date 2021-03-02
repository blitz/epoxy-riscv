        // TODO Include state.hpp and have it export the number of processes.

        .section .data
        .global USER_SATPS, USER_PCS

        // TODO Hardcode 16 address spaces and threads for now.

        // SATP values for each process.
USER_SATPS:
        .fill (8 * 16)

        // Program counters for each thread.
USER_PCS:
        .fill (8 * 16)
