## simple "weather station"

Uses BME280 sensor for temperature and humidity reading, 
DS1307 RTC for time and date, and SSD1306 OLED as display. 

![](weatherstation.jpg)

Shared-bus is used to share the I2C between the three devices.

Requires patched bme280 driver, otherwise will not compile with the STM32Fxx crate

Patch consists of removing the I2c Read trait, otherwise the compiler will return the following error: 

~~~~
67 |         let mut bme280 = BME280::new(i2c, bme280_i2c_addr, delay);    
   |                          ^^^^^^^^^^^ the trait `hal::prelude::_embedded_hal_blocking_i2c_Read` is not implemented for `hal::i2c::I2c<hal::stm32::I2C1, hal::gpio::gpioa::PA9<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpioa::PA10<hal::gpio::Alternate<hal::gpio::AF4>>>`
   |
   = note: required by `bme280::BME280::<I2C, D>::new`
~~~~

(ERROR SHOWN FOR STM32F030xx)

Currently will not fit into memory on STM32F030 board.

TO DO:

* add decimal points and better formatting
* add pressure reading
* try to make code smaller to fit on STM32F030
* propose a PR with the patch