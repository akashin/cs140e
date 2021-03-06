#![feature(lang_items, llvm_asm)]
#![no_builtins]
#![no_std]

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { llvm_asm!("nop" :::: "volatile"); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    // Set GPIO Pin 16 as output.
    GPIO_FSEL1.write_volatile(0b001 << 18);

    loop {
        // Continuously set and clear GPIO 16.
        GPIO_SET0.write_volatile(1 << 16);
        spin_sleep_ms(1000);
        GPIO_CLR0.write_volatile(1 << 16);
        spin_sleep_ms(1000);
    }
}
