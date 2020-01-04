	.section .data
	.extern USER_SATPS

USER_SATPS:
	// TODO Hardcode 16 address spaces for now
	.fill (8 * 16)
