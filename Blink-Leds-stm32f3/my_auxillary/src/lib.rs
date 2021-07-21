
#![no_std]
/// This program is lib for main program which provide the implementation to blink leds in stm32f3 board.
pub use  panic_itm;
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    stm32f3xx_hal,
    leds::Leds,
    switch_hal::{ActiveHigh, OutputSwitch, Switch},
};
pub use stm32f3xx_hal::{
    gpio::{gpioe, Output, PushPull},
    prelude::*,
    stm32,
    delay::Delay
};
pub use cortex_m::peripheral::SYST;

/// This program contains the implementation for the blinking of LEDS project.
///
///
/// LedArray is used to provide the leds an array structure.
pub type  LedArray= [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];
/// Fn mycrate() -> this function contains the device peripheral, core peripheral and delay.
///
/// #Return
///
/// Tuple of (LedArray, Delay)
/// LedArray-> All the leds in output mode as array.
/// Delay -> providing the time period to stop the process for 1 sec)
pub fn mycrate() -> (LedArray,Delay) {
    // Setting up device peripheral to get the access of the f3 board peripherals.
    let device_peripheral = stm32::Peripherals::take().unwrap();
    // Setting up the Reset & Clock Control to get the access of system clock which will be used
    // to generate delay.
    let mut reset_clock_control = device_peripheral.RCC.constrain();
    // Setting up the core peripheral to control the clocks for delay.
    let core_peripheral = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripheral.FLASH.constrain();
    let clocks = reset_clock_control.cfgr.freeze(&mut flash.acr);
    // Using delay to stop the process for 1 sec.
    let delay = Delay::new(core_peripheral.SYST, clocks);
    // Taking access of f3 Board port-e peripheral which contains the led's pin address.
    // Splits the GPIOE peripheral into 16 independent pins
    let mut gpioe = device_peripheral.GPIOE.split(&mut reset_clock_control.ahb);
    // Taking access of all the LED pins of the f3 Board and setting them as "ouput" mode.
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );
    // providing leds in an array type using into_array() function.
    (leds.into_array(),delay)
}