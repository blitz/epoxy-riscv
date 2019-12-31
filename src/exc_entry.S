        .section .text
        .extern exc_entry

        .global asm_exc_entry
        .align 2
asm_exc_entry:

        // TODO We actually need to save state here.

        j exc_entry
