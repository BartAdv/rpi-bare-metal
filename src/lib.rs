#![feature(asm, core_intrinsics, lang_items)]
#![no_std]

// http://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

const GPIO_BASE: u32 = 0x20200000;
// offsets
const GPFSEL1: u32 = GPIO_BASE + 4; // ???
const GPFSEL4: u32 = GPIO_BASE + 0x10;
const GPSET0: u32 = 0x2020001C; // GPIO_BASE + 0x1C;
const GPSET1: u32 = GPIO_BASE + 0x20;
const GPCLR0: u32 = GPIO_BASE + 0x28;
const GPCLR1: u32 = GPIO_BASE + 0x2C;

const ACT_LED: u32 = 16;

const UART_DR: u32 = GPIO_BASE + 0x1000;
const UART_FR: u32 = GPIO_BASE + 0x1018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}

#[no_mangle]
pub extern fn kmain(r0: u32, r1: u32, atags: u32) {
    loop {
        sleep(1000000);
        unsafe { volatile_store(GPSET0 as *mut u32, 1 << ACT_LED); }
        sleep(1000000);
        unsafe { volatile_store(GPCLR0 as *mut u32, 1 << ACT_LED); }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}