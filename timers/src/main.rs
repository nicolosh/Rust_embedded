#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use lm3s6965::interrupt;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// General Purpose Timer Module (GPTM)
//
// GPTM Control
const GPTMCTL: u32 = 0x4003000c;
// GPTM Configuration
const GPTMCFG: u32 = 0x40030000;
// GPTM Timer A Mode Register
const GPTMTAMR: u32 = 0x40030004;
// GPTM Timer A Interval Load Register
const GPTMTAILR: u32 = 0x40030028;
// GPTM Interrupt Mask Register
const GPTMIMR: u32 = 0x40030018;

// Run-Mode Clock Configuration (RCC)
const RCC: u32 = 0x400fe060;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");
    
    // Configure GPTM control registers
    unsafe {
        // Disable the timer
        *(GPTMCTL as *mut u32) = 0b0;
        // Set the timer to 32 bit configuration
        *(GPTMCFG as *mut u32) = 0x0;
        // Put the timer in periodic mode
        *(GPTMTAMR as *mut u32) = 0x2;
        // Set the reload value (i.e the period of the timer)
        *(GPTMTAILR as *mut u32) = 25_000_000;
        // Enable the interrupt
        *(GPTMIMR as *mut u32) = 0b1;
        // Enable the timer
        *(GPTMCTL as *mut u32) = 0b1;
    };

    let div = 0x3;
    let sysdiv = div << 23;
    unsafe {
        // Set the RCC value
        let rcc = {
            let orig = *(RCC as *const u32);
            orig | sysdiv
        };
        *(RCC as *mut u32) = rcc;
    };

    loop {}
}

#[interrupt]
fn TIMER_0A() {
    hprintln!("ugh, woke up :(")
}
