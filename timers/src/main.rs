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
// Prescaler value
const SYSCTL_SYSDIV_16: u32 = 0xF; // 12.5 MHz
const SYSCTL_SYSDIV_12: u32 = 0xB; // 16.67 MHz

// CPU frequency (12.5 MHz by default)
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");

    // Set the prescaler value
    unsafe {
        let sysdiv = SYSCTL_SYSDIV_16 << 23;
        let orig = *(RCC as *const u32);
        let mask = !0b1111 << 23;
        let rcc = (orig & mask) | sysdiv;
        *(RCC as *mut u32) = rcc;
    };

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    systick.enable_interrupt();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(CPU_FREQ);
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
