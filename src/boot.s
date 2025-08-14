.section .text._start
.global _start
_start:
    # Print initial string
    li t0, 0x10000000    # UART base address
    la t1, init_string   # Load address of string
print_loop:
    lb t2, (t1)          # Load byte from string
    beq t2, zero, done   # Exit if null terminator
    sb t2, (t0)          # Write byte to UART
    addi t1, t1, 1       # Next character
    j print_loop
done:
    # Set up stack pointer
    la sp, _stack_start
    # Call Rust loader_init
    call loader_init
    # Infinite loop in case loader_init returns
    j .

.section .rodata
init_string:
    .string "ZYXWVUQTSPR\n"
