#![no_main]
#![no_std]

#[allow(unused)]
// use panic_halt;

use stm32f4xx_hal as hal;
use ws2812_timer_delay as ws2812;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::stm32;
use crate::hal::time::*;
use crate::hal::timer::*;
use crate::ws2812::Ws2812;
use hal::prelude::*;

use embedded_hal::digital::OutputPin;

use cortex_m::peripheral::Peripherals;

use smart_leds::{brightness, SmartLedsWrite, RGB8, gamma};
// use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_core::{self, rtt_init_print, rprintln};

use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::time::MegaHertz;
use stm32f4xx_hal::hal::blocking::delay::DelayMs;

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // Constrain clocking registers
        let mut flash = p.FLASH;
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(56.mhz()).freeze();


        let gpioa = p.GPIOA.split();
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_push_pull_output();

        // let ss = gpioa.pa4.into_push_pull_output();
        // let ss_pin = embedded_hal::digital::v1_compat::OldOutputPin::new(ss);

        let timer = Timer::tim1(p.TIM1, MegaHertz(3), clocks);

        const NUMLEDS: usize = 30;
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        let mut data: [RGB8; NUMLEDS] = [RGB8::default(); NUMLEDS];
        let empty: [RGB8; NUMLEDS] = [RGB8::default(); NUMLEDS];
        let mut ws = Ws2812::new(timer, mosi);
        let mut start = 15;
        rprintln!("start {}", start);
        loop {
            // if let Err(e) = ws.write(empty.iter().cloned()){
            //     rprintln!("error {:?}", e);
            // };


            for i in 0..data.len()-1 {
                data[i] = wheel(((255 / NUMLEDS) * ((i+start)%NUMLEDS)) as u8);
            }
            start = start+1;
            if start >= NUMLEDS {
                start = 0;
            }


            // data[0] = RGB8 {
            //     r: 0xff,
            //     g: 0x0,
            //     b: 0xff
            // };
            // data[1] = RGB8 {
            //     r: 0xff,
            //     g: 0xff,
            //     b: 0x00,
            // };
            // data[2] = RGB8 {
            //     r: 0x0,
            //     g: 0xff,
            //     b: 0x0
            // };
            // data[3] = RGB8 {
            //     r: 0x0,
            //     g: 0x0,
            //     b: 0xff,
            // };
            rprintln!("Write colors. {}", start);
            for start in 0..NUMLEDS-1 {
                for i in 0..data.len()-1 {
                    data[i] = wheel(((255 / NUMLEDS) * ((i+start)%NUMLEDS)) as u8);
                }
                if let Err(e) = ws.write(brightness(gamma(data.iter().cloned()),0x0f)) {
                    //if let Err(e) = ws.write(data.iter().cloned()) {
                    rprintln!("error {:?}", e);
                };
            }

            if let Err(e) = ws.write(brightness(gamma(data.iter().cloned()),0x0f)) {
                //if let Err(e) = ws.write(data.iter().cloned()) {
                rprintln!("error2 {:?}", e);
            };

            rprintln!("Sleep.");
            delay.delay_ms(10000 as u16);
            rprintln!("Write zeros.");
            if let Err(e) = ws.write(empty.iter().cloned()){
                rprintln!("error {:?}", e);
            };
            rprintln!("Sleep.");
            //delay.delay_ms(2000 as u16);
        }
    }
    loop {
        continue;
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
