#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Run-Mode Clock Configuration (RCC)
const RCC: u32 = 0x400FE060;
// Prescaler values
const SYSCTL_SYSDIV_16: u32 = 16; // 12.5 MHz
const SYSCTL_SYSDIV_12: u32 = 12; // 16.67 MHz

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");

    let div = SYSCTL_SYSDIV_12;
    let freq = match div {
        SYSCTL_SYSDIV_12 => 16_670_000,
        SYSCTL_SYSDIV_16 => 12_500_000,
        _ => 12_500_000,
    };

    // Set the RCC value
    unsafe {
        let sysdiv = div << 23;
        let rcc = {
            let orig = *(RCC as *const u32);
            orig | sysdiv
        };
        *(RCC as *mut u32) = rcc;
    };

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    systick.enable_interrupt();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(freq);
    systick.clear_current();
    systick.enable_counter();

    loop {
        asm::wfi();
    }
}

#[exception]
fn SysTick() {
    hprintln!("ugh, woke up :(")
}
