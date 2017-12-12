// http://wiki.osdev.org/Raspberry_Pi_Bare_Bones

// To keep this in the first portion of the binary.
.section ".text.boot"
 
// Make _start global.
.globl _start

// Entry point for the kernel.
// r15 -> should begin execution at 0x8000.
// r0 -> 0x00000000
// r1 -> 0x00000C42
// r2 -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main
_start:
  ldr r0,=0x20200000
  mov r1,#1
  lsl r1,#18
  str r1,[r0,#4]
  mov r1,#1
  lsl r1,#16
  str r1,[r0,#40]

	// Setup the stack.
	mov sp, #0x8000

	// Clear out bss.
	ldr r4, =__bss_start
	ldr r9, =__bss_end
	mov r5, #0
	mov r6, #0
	mov r7, #0
	mov r8, #0
	b       2f

1:
	// store multiple at r4.
	stmia r4!, {r5-r8}

	// If we are still below bss_end, loop.
2:
	cmp r4, r9
	blo 1b

	// Call kmain
	ldr r3, =kmain
	blx r3

	// halt
halt:
	wfe
	b halt