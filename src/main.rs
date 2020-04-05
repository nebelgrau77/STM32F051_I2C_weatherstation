//! simple clock using DS1307 RTC and SSD1306 in TerminalMode
//!  
//! Best results when using `--release`.


#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
//extern crate panic_halt;
extern crate panic_semihosting;
extern crate stm32f0xx_hal as hal;
extern crate shared_bus;

use cortex_m_rt::entry;
use ssd1306::{prelude::*, Builder as SSD1306Builder};

use ds1307::{DateTime, Hours, DS1307};

use bme280::BME280;

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c,
    delay::Delay,    
};

use core::fmt;
use core::fmt::Write;
use arrayvec::ArrayString;

const BOOT_DELAY_MS: u16 = 100;

#[entry]
fn main() -> ! {

    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);
        
        let mut delay = Delay::new(cp.SYST, &rcc);

        delay.delay_ms(BOOT_DELAY_MS);

        let gpiob = p.GPIOB.split(&mut rcc);
        let scl = gpiob.pb8.into_alternate_af1(cs);
        let sda = gpiob.pb7.into_alternate_af1(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

        let manager = shared_bus::CortexMBusManager::new(i2c);

        let mut disp: TerminalMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(manager.acquire()).into();
        
        disp.init().unwrap();

        disp.clear().unwrap();

        let mut rtc = DS1307::new(manager.acquire());

        // BME280 sensor initiation
        let bme280_i2c_addr = 0x76;
        let mut bme280 = BME280::new(manager.acquire(), bme280_i2c_addr, delay);    
        bme280.init().unwrap();


        loop {
            
            let datetime = rtc.get_datetime().unwrap();

            let mut buffer = ArrayString::<[u8; 64]>::new();

            let measurements = bme280.measure().unwrap();

            let celsius = measurements.temperature;
            let humidity = measurements.humidity;
           
            match datetime.hour {
                Hours::H24(h) => format(&mut buffer, h as u8, datetime.minute as u8, datetime.second as u8,
                                        (datetime.year - 2000) as u8, datetime.month as u8, datetime.day as u8,
                                        celsius as u8, humidity as u8),
                
                _ => format(&mut buffer, 0,0,0,0,0,0,0,0),

                };
        
            disp.write_str(buffer.as_str());
        
            // just wait for many cycles
            //cortex_m::asm::delay(1 * 48_000_000);

            }

       
    });
    
}

    loop {continue;}
    
}


fn format(buf: &mut ArrayString<[u8; 64]>, hrs: u8, mins: u8, secs: u8, year: u8, mon: u8, day: u8, temp: u8, hum: u8) {
    fmt::write(buf, format_args!("    {:02}:{:02}:{:02}        {:02}/{:02}/{:02}         T: {:02}C          H: {:02}%     ",
     hrs, mins, secs, year, mon, day, temp, hum)).unwrap();
}

