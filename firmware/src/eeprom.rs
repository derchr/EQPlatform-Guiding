//! This module handles all the necessary EEPROM functionality.
//! The EEPROM will hold the last velocity set by UART,
//! so it can be automatically loaded on startup.
//! It also holds the number of runtimes of the program.

use embedded_time::duration::*;

pub fn read_waiting_time() -> Microseconds {
    Microseconds(10u32)
}

#[allow(dead_code)]
pub fn write_waiting_time(_time: Microseconds) {

}

pub fn increment_runtime() {

}
