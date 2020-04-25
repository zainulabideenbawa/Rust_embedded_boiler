#![no_main]
#![no_std]
use cortex_m::asm;
use cortex_m_rt::entry;
use f3::hal::delay::Delay;
use f3::hal::gpio;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::hal::stm32f30x::{GPIOA, GPIOB, RCC, TIM4};
#[allow(unused)]
use panic_halt;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);
    // Turn on PORTB
    unsafe {
        let mut rcc_block = &*RCC::ptr();
        rcc_block.ahbenr.write(|w| w.iopben().enabled());
        
    }

    

        loop {} 
}
