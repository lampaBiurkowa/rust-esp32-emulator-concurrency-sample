#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

const EEPROM_ADDRESS: u8 = 0x50;
const BYTE_TO_SET: u16 = 0;
const VALUE_TO_WRITE: u8 = 0x46;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Initializing...").unwrap();

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000
    );

    let mut delay = arduino_hal::Delay::new();
    write_byte_eeprom(&mut i2c, &mut delay, BYTE_TO_SET, VALUE_TO_WRITE).unwrap();

    let mut read_byte: u8 = 0;
    read_byte_eeprom(&mut i2c, BYTE_TO_SET, &mut read_byte).unwrap();

    ufmt::uwriteln!(&mut serial, "Read byte: 0x{:X}\r", read_byte).unwrap();

    loop {}
}

fn write_byte_eeprom(
    i2c: &mut arduino_hal::I2c,
    delay: &mut arduino_hal::Delay,
    address: u16,
    data: u8,
) -> Result<(), arduino_hal::i2c::Error> {
    let (addr_high, addr_low) = to_high_low(address);
    i2c.write(EEPROM_ADDRESS, &[addr_high, addr_low, data])?;
    delay.delay_ms(5u16); //otherwise it ends prematurly...
    Ok(())
}

fn read_byte_eeprom(
    i2c: &mut arduino_hal::I2c,
    address: u16,
    buffer: &mut u8,
) -> Result<(), arduino_hal::i2c::Error> {
    let (addr_high, addr_low) = to_high_low(address);
    i2c.write(EEPROM_ADDRESS, &[addr_high, addr_low])?;
    i2c.read(EEPROM_ADDRESS, core::slice::from_mut(buffer))?;
    Ok(())
}

fn to_high_low(address: u16) -> (u8, u8) {
    let addr_high = (address >> 8) as u8;
    let addr_low = (address & 0xFF) as u8;
    (addr_high, addr_low)
}