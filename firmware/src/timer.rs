//! The 16-bit (0-65535) timer will be set up in the Clear Timer Compare mode.
//! This means the MCU automatically calls the ISR when a specific value
//! of the timer is reached. It will also reset the timer automatically. The
//! prescaler will ensure that only every 64-th clock tick the timer will actually
//! increase in value, resulting in a maximum time period of 0.26214 seconds.

use crate::TIMER_STRUCTURE;
use atmega328p_hal::prelude::*;
use core::ops::DerefMut;
use embedded_time::duration::*;

pub fn init() {
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut timer_struct) = TIMER_STRUCTURE.borrow(cs).borrow_mut().deref_mut() {
            // Prescaler 64, CTC (Clear Timer Compare) enable
            // TCCR1B |= (1<<CS10) | (1<<CS11) | (1<<WGM12);

            // Timer Configuration:
            // - WGM = 4: CTC mode (Clear Timer on Compare Match)
            // - Prescaler 64
            // - OCR1A = 15624
            let tmr1 = &mut timer_struct.tc1;
            tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));

            tmr1.tccr1b
                .write(|w| w.cs1().prescale_64().wgm1().bits(0b01));
        }
    });
}

pub fn set_timer_status(active: bool) {
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut timer_struct) = TIMER_STRUCTURE.borrow(cs).borrow_mut().deref_mut() {
            let tmr1 = &mut timer_struct.tc1;

            // Compare Interrupt enable/disable
            // TIMSK1 |= (1<<OCIE1A);

            if active {
                tmr1.timsk1.write(|w| w.ocie1a().set_bit());
            } else {
                tmr1.timsk1.write(|w| w.ocie1a().clear_bit());
                // When we disable the timer, we also want to ensure that the pin is set to low.
                timer_struct.pin.set_low().void_unwrap();
            }
        }
    });
}

pub fn set_duration(duration: Microseconds) {
    let time = *duration.integer();

    // The register is exactly 16 bits wide.
    let compare_value: u16 = (time / 4) as u16;

    avr_device::interrupt::free(|cs| {
        if let Some(ref mut timer_struct) = TIMER_STRUCTURE.borrow(cs).borrow_mut().deref_mut() {
            timer_struct
                .tc1
                .ocr1a
                .write(|w| unsafe { w.bits(compare_value) });
        }
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut timer_struct) = TIMER_STRUCTURE.borrow(cs).borrow_mut().deref_mut() {
            if timer_struct.pin_is_high {
                timer_struct.pin.set_low().void_unwrap();
                timer_struct.pin_is_high = false;
                //set_duration(Microseconds(timer_struct.waiting_time.integer() / 2));
            } else {
                timer_struct.pin.set_high().void_unwrap();
                timer_struct.pin_is_high = true;
                //set_duration(Microseconds(timer_struct.waiting_time.integer() / 2));
            }
        }
    });
}
