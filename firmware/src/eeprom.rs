//! This module handles all the necessary EEPROM functionality.
//! The EEPROM will hold the last velocity set by UART,
//! so it can be automatically loaded on startup.
//! It also holds the number of runtimes of the program.
//! The Atmega328p chip has a word size of 8 Bit.

use atmega328p_hal as hal;
use hal::pac::EEPROM;
use embedded_time::duration::*;

const BASE_ADDR_STARTUPS: u16 = 0x0000;
const BASE_ADDR_TIME: u16 = 0x00F0;

pub fn read_waiting_time(eeprom_registers: &EEPROM) -> Microseconds {
    let mut time = [0 as u8; 4];
    
    for (i, word) in time.iter_mut().enumerate() {
        *word = read_word(BASE_ADDR_TIME + i as u16, eeprom_registers);
    }
    
    Microseconds(u8_to_u32(time))
}

pub fn write_waiting_time(time: Microseconds, eeprom_registers: &EEPROM) {

    let time = time.integer().to_be_bytes();
    
    for (i, word) in time.iter().enumerate() {
        write_word(*word, BASE_ADDR_TIME + i as u16, eeprom_registers);
    }

}

pub fn increment_startups(eeprom_registers: &EEPROM) {

    let startups = read_startups(eeprom_registers) + 1;
    let startups = startups.to_be_bytes();
    
    for (i, word) in startups.iter().enumerate() {
        write_word(*word, BASE_ADDR_STARTUPS + i as u16, eeprom_registers);
    }

}

pub fn read_startups(eeprom_registers: &EEPROM) -> u32 {

    let mut startups = [0 as u8; 4];
    
    for (i, word) in startups.iter_mut().enumerate() {
        *word = read_word(BASE_ADDR_STARTUPS + i as u16, eeprom_registers);
    }

    u8_to_u32(startups)

}

fn read_word(address: u16, eeprom_registers: &EEPROM) -> u8 {

    // Write address
    // SAFETY:
    // No other interrupt will access this register in any way
    // to it is safe to write raw bits into it.
    eeprom_registers.eear.write(|w| unsafe { w.bits(address) } );
    
    // Enable read operation
    eeprom_registers.eecr.write(|w| w.eere().set_bit());
    
    // Read the bits from the register
    eeprom_registers.eedr.read().bits()
}

fn write_word(word: u8, address: u16, eeprom_registers: &EEPROM) {

    // Wait until writing is possible.
    while eeprom_registers.eecr.read().eepe().bit_is_set() {};
    
    // Write address
    // SAFETY:
    // No other interrupt will access this register in any way
    // to it is safe to write raw bits into it.
    eeprom_registers.eear.write(|w| unsafe { w.bits(address) } );
    
    // Write data
    // SAFETY:
    // No other interrupt will access this register in any way
    // to it is safe to write raw bits into it.
    eeprom_registers.eedr.write(|w| unsafe { w.bits(word) } );
    
    // Enable master write
    eeprom_registers.eecr.modify(|_, w| w.eempe().set_bit() );
    // Enable write
    eeprom_registers.eecr.modify(|_, w| w.eepe().set_bit() );

}

fn u8_to_u32(number_array: [u8; 4]) -> u32 {

    let mut result;
    
    result = (number_array[0] as u32) << 24;
    result += (number_array[1] as u32) << 24;
    result += (number_array[2] as u32) << 24;
    result += number_array[3] as u32;
    
    result

}
