//! Here live the interrupt service routines used for serial commutication
//! and useful functions for the serial port.

use crate::{SerialBuffer, Usart0Writer, SERIAL_BUFFER};

use core::ops::DerefMut;

use atmega328p_hal as hal;
use hal::clock::MHz16;
use hal::port::*;
use hal::usart::*;

use embedded_time::duration::*;

use staticvec::StaticString;

pub enum InputVariant {
    Track,
    TrackNewTime(Microseconds),
    Hold,
    FastForward(bool),
    SetDefault,
    Status,
    Reset,
    Invalid,
}

pub struct SerialHandler {
    usart0_tx: Usart0Writer,
}

impl SerialHandler {
    pub fn new(usart_interface: hal::pac::USART0, mut portd: portd::Parts) -> Self {
        let baudrate = Baudrate::<MHz16>::new(57600);

        let mut usart0 = Usart0::new(
            usart_interface,
            portd.pd0,
            portd.pd1.into_output(&mut portd.ddr),
            baudrate,
        );

        // Enable UART interrupts
        usart0.listen(Event::RxComplete);

        // Future functionality: Add a TX buffer to write data using
        // the USART_UDRE interrupt, which should make things smoother.
        // usart0.listen(Event::DataRegisterEmpty);

        let (usart0_rx, usart0_tx) = usart0.split();

        avr_device::interrupt::free(|cs| {
            SERIAL_BUFFER.borrow(cs).replace(Some(SerialBuffer {
                usart0_rx,
                buffer: StaticString::new(),
                is_complete: false,
            }));
        });

        Self { usart0_tx }
    }

    pub fn handle_input(&mut self) -> Option<InputVariant> {
        let mut parse_result = None;
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut serial_buffer) = SERIAL_BUFFER.borrow(cs).borrow_mut().deref_mut() {
                if serial_buffer.is_complete {
                    let buffer = serial_buffer.buffer.as_str();
                    parse_result = Some(parse_input(buffer));
                    ufmt::uwriteln!(self.usart0_tx, "Got: {}", buffer).ok();
                    serial_buffer.buffer.clear();
                    serial_buffer.is_complete = false;
                }
            }
        });
        parse_result
    }

    pub fn write_str(&mut self, string: &str) {
        ufmt::uwrite!(self.usart0_tx, "{}", string).ok();
    }

    pub fn write_number<T: num::Integer + ufmt::uDisplay>(&mut self, value: T) {
        // For some reason uwrite does only support 16bit integers?
        ufmt::uwrite!(self.usart0_tx, "{}", value).ok();
    }

    pub fn send_status(&mut self, current_time: u32, default_time: u32, starts: u32) {
        ufmt::uwriteln!(
            self.usart0_tx,
            "\n\n\
            ~~~~~~~~~~ EQPlatform-PulseGuiding ~~~~~~~~~~\n\
            ~                                           ~\n\
            ~          Firmware-Version: {}          ~\n\
            ~           Current Velocity: {}         ~\n\
            ~           Default Velocity: {}         ~\n\
            ~           Number of starts: {}            ~\n\
            ~                                           ~\n\
            ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
            env!("CARGO_PKG_VERSION"),
            current_time,
            default_time,
            starts,
        )
        .ok();
    }
}

fn parse_input(input: &str) -> InputVariant {
    /*  This is a mess:
    I really don't know why but matching against the &str type like
    'Err("track")' did not work in all cases. For example "hold" was
    working, but "track" not. Also passing "track" directly into the
    parse_input() function did work as expected. However creating a
    StaticString from "track" and then giving back the &str type did
    not work. Only checking the first char is a workaround. */

    match input.parse::<u32>().map_err(|_| input.chars().nth(0)) {
        // First lets see if our string is a number. Then the
        // user meant to send a new velocity to the tracker.
        Ok(duration) => InputVariant::TrackNewTime(Microseconds(duration)),

        // Alternatively the user can send a "t" to resume tracking.
        Err(Some('t')) => InputVariant::Track,

        // If the input is "+" or "-" the user wants to enter the
        // fast forward mode. The direction is set accordingly.
        Err(Some('+')) => InputVariant::FastForward(true),
        Err(Some('-')) => InputVariant::FastForward(false),

        // When the input string is "h" the user wants
        // the tracker to immediately halt.
        Err(Some('h')) => InputVariant::Hold,

        // When the user has found a good velocity he
        // can use the "d" command to save the current
        // velocity to the EEPROM.
        Err(Some('d')) => InputVariant::SetDefault,

        // Print some status info.
        Err(Some('s')) => InputVariant::Status,

        // Reset chip.
        Err(Some('r')) => InputVariant::Reset,

        // If all checks fail the user hasn't inputted anything valid.
        _ => InputVariant::Invalid,
    }
}

/// Here live the interrupt service routines needed for serial communication.
mod serial_isr {
    use atmega328p_hal as hal;
    use core::ops::DerefMut;
    use hal::prelude::*;

    #[avr_device::interrupt(atmega328p)]
    fn USART_RX() {
        use crate::SERIAL_BUFFER;
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut serial_buffer) = SERIAL_BUFFER.borrow(cs).borrow_mut().deref_mut() {
                let byte = serial_buffer.usart0_rx.read().unwrap();
                if byte == b'\n' {
                    serial_buffer.is_complete = true;
                } else {
                    // Try to push. When the buffer is full, simply ignore all new characters.
                    if let Some(character) = core::char::from_u32(byte as u32) {
                        serial_buffer.buffer.try_push(character).ok();
                    }
                }
            }
        });
    }
}
