//! Here live the interrupt service routines used for serial commutication
//! and useful functions for the serial port.

use crate::{
    SerialBuffer,
    SERIAL_BUFFER,
    Usart0Writer,
    InputVariant
};

use core::ops::DerefMut;

use atmega328p_hal as hal;
use hal::clock::MHz16;
use hal::usart::*;
use hal::port::*;

use embedded_time::duration::*;

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
        // the USART_UDRE interrupt, wich should make things smoother.
        //usart0.listen(Event::DataRegisterEmpty);

        let (usart0_rx, usart0_tx) = usart0.split();

        avr_device::interrupt::free(|cs| {
            SERIAL_BUFFER.borrow(cs).replace(Some(
                SerialBuffer {
                    usart0_rx,
                    buffer: arrayvec::ArrayString::<[u8; 64]>::new(),
                    is_complete: false,
                }
            ));
        });

        Self {
            usart0_tx,
        }
    }

    pub fn handle_input(&mut self) -> Option<InputVariant> {
        let mut input_vec = arrayvec::ArrayString::<[u8; 64]>::new();
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut serial_buffer) = SERIAL_BUFFER.borrow(cs).borrow_mut().deref_mut() {
                if serial_buffer.is_complete {
                    input_vec = serial_buffer.buffer.clone();
                    ufmt::uwriteln!(self.usart0_tx, "Got: {}", input_vec.as_str()).ok();
                    serial_buffer.buffer.clear();
                    serial_buffer.is_complete = false;
                }
            }
        });
        parse_input(input_vec.as_str())
    }

    pub fn write_str(&mut self, string: &str) {
        ufmt::uwriteln!(self.usart0_tx, "{}", string).ok();
    }

    pub fn send_status(&mut self) {
        ufmt::uwriteln!(self.usart0_tx,
            "\n\n\
            ~~~~~~~~~~ EQPlatform-PulseGuiding ~~~~~~~~~~\n\
            ~                                           ~\n\
            ~          Firmware-Version: {}          ~\n\
            ~           Current Velocity: {}            ~\n\
            ~           Default Velocity: {}            ~\n\
            ~           Number of starts: {}            ~\n\
            ~                                           ~\n\
            ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
            env!("CARGO_PKG_VERSION"),
            10,
            12,
            12,
        ).ok();
    }

}

fn parse_input(input: &str) -> Option<InputVariant>{
    match input.parse() {
        // First lets see if our string is a number. Then the
        // user meant to send a new velocity to the tracker.
        Ok(time) => Some(InputVariant::Track(Microseconds(time))),

        // If the input is "f+" or "f-" the user wants to enter the
        // fast forward mode. The direction is set accordingly.
        Ok("f+") => Some(InputVariant::FastForward(true)),
        Ok("f-") => Some(InputVariant::FastForward(false)),

        // When the input string is "h" the user wants
        // the tracker to immediately halt.
        Ok("h") => Some(InputVariant::Hold),

        // When the user has found a good velocity he
        // can use the "d" command to save the current
        // velocity to the EEPROM.
        Ok("d") => Some(InputVariant::SetDefault),

        // Print some status info.
        Ok("s") => Some(InputVariant::Status),

        // If all checks fail the user hasn't yet inputted anything.
        _ => None
    }
}

/// Here live the interrupt service routines needed for serial communication.
mod serial_isr {
    use atmega328p_hal as hal;
    use hal::prelude::*;
    use core::ops::DerefMut;

    #[avr_device::interrupt(atmega328p)]
    fn USART_RX() {
        use crate::SERIAL_BUFFER;
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut serial_buffer) = SERIAL_BUFFER.borrow(cs).borrow_mut().deref_mut() {
                let new_char = serial_buffer.usart0_rx.read().unwrap();
                if new_char == b'\n' {
                    serial_buffer.is_complete = true;
                } else {
                    // Try to push. When the buffer is full, simply ignore all new characters.
                    serial_buffer.buffer.try_push(new_char as char).ok();
                }
            }
        });
    }

    #[avr_device::interrupt(atmega328p)]
    fn USART_UDRE() {
        /*use crate::{LED_PIN, SERIAL};
        avr_device::interrupt::free(|cs| {
            if let Some(ref mut pin) = LED_PIN.borrow(cs).borrow_mut().deref_mut() {
                pin.toggle();
            }
        });*/
    }
}