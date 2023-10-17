#![no_main]
#![no_std]

use core::mem::size_of;
// gives a panic_handler with a endless loop
use panic_halt as _;

// Base addr of RCC peripheral
const RCC_BASE: *const u32 = 0x4002_1000 as *const _; // _ => wild card works here because type is given
                                                      // Offset of the AHBENR Register
const RCC_AHBENR: usize = 0x14;
// Base addr of GPIOE peripheral
const GPIOE_BASE: *const u32 = 0x4800_1000 as *const _;
// Offsets of the Registers relative to base
const GPIO_MODER_OFFSET: usize = 0x00;
const _GPIO_OTYPER_OFFSET: usize = 0x04;
const _GPIO_OSPEEDR_OFFSET: usize = 0x08;
const _GPIO_PUPDR_OFFSET: usize = 0x0c;
const _GPIO_IDR_OFFSET: usize = 0x10;
const _GPIO_ODR_OFFSET: usize = 0x14;
const GPIO_BSSR_OFFSET: usize = 0x18;
#[cortex_m_rt::entry]
fn main() -> ! {
    // Create non mutable Pointer to AHBENR Register
    let ahbenr = unsafe { RCC_BASE.add(RCC_AHBENR / size_of::<usize>()) };
    // cast to mutable pointer - casting is SAFE
    let ahbenr = ahbenr.cast_mut();

    // Read Register by deref pointer
    let mut reg = unsafe { *ahbenr }; // Deref a Pointer is NOT SAFE

    // Modify reg
    // GPIOE EN Bit 21
    reg |= 1 << 21;
    // Write new value to Register
    unsafe {
        ahbenr.write_volatile(reg);
    }
    // Create Pointer to MODER Register
    let moder = unsafe {
        // b01 => output
        GPIOE_BASE
            .add(GPIO_MODER_OFFSET / size_of::<usize>())
            .cast_mut()
    };

    // Read Register
    let mut reg = unsafe { *moder };
    // 0b01 => Output Pin 10 => 20 Shifts
    reg |= 0b01 << 20;
    // Write new value to Register
    unsafe {
        moder.write_volatile(reg);
    }

    // BSSR Register no need to Read back
    let bssr = unsafe {
        // Create Pointer to GPIOE BSSR
        GPIOE_BASE
            .add(GPIO_BSSR_OFFSET / size_of::<usize>())
            .cast_mut()
    };

    unsafe {
        // Write Set Bit Pin 10
        bssr.write_volatile(1 << 10);
    }

    loop {
        // ~1s Blink pattern
        for _i in 0..1_000_000 {
            cortex_m::asm::nop();
        }

        unsafe {
            // Write reset Bit Pin 10
            bssr.write_volatile(1 << 26);
        }
        for _i in 0..1000000 {
            cortex_m::asm::nop();
        }

        unsafe {
            // Write Set Bit Pin 10
            bssr.write_volatile(1 << 10);
        }
    }
}
