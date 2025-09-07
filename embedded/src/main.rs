#![no_std]
#![no_main] // since we use no_std, we need to disable all the normal entry points

use core::panic::PanicInfo;
// use cortex_m::asm;
use cortex_m::peripheral::syst::SystClkSource; // for SysTick
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
// use cortex_m_rt::exception;

use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// CPU f = 12.5 MHz default
const CPU_FREQ: u32 = 12_500_000;

#[entry]
fn main() -> ! {
    hprintln!("Starting program!"); // semihosting

    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST; // system timer
    systick.enable_interrupt(); // enables system timer interrupt
    systick.set_clock_source(SystClkSource::Core); // set clock source to core clock
    systick.set_reload(CPU_FREQ); // set reload value to 1 second (12.5 million cycles)
    systick.clear_current(); // clear systimer current value so we start at the actual reload value ARR
    systick.enable_counter(); // start the counter systimer

    let mut counter = 0;
    // equivalent of while(1) {} in Stm32cube IDE
    loop {
        // asm::wfi(); // wait for interrupt so that it prints ugh, woke up :( every 1 second
        counter += 1;
        hprintln!("Counter value every loop iteration: {}", counter);
    }
}

//#[exception]
//fn SysTick() {
//    hprintln!("ugh, woke up :(");
//}
