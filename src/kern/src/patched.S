        // TODO Include state.hpp and have it export the number of processes.

        .section .data
	.global USER_SATPS

USER_SATPS:
	// TODO Hardcode 16 address spaces for now
	.fill (8 * 16)
