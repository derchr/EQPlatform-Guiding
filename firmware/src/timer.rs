//! The 16-bit (0-65535) timer will be set up in the Clear Timer Compare mode.
//! This means the MCU automatically calls the ISR when a specific value
//! of the timer is reached. It will also reset the timer automatically. The
//! prescaler will ensure that only every 64-th clock tick the timer will actually
//! increase in value, resulting in a maximum time period of 0.26214 seconds.

use atmega328p_hal::pac::TC1;
use atmega328p_hal::prelude::*;
use core::ops::DerefMut;
use embedded_time::duration::*;

pub fn init(tmr1: &TC1) {
    // Prescaler 64, CTC (Clear Timer Compare) enable
    // TCCR1B |= (1<<CS10) | (1<<CS11) | (1<<WGM12);

    // Timer Configuration:
    // - WGM = 4: CTC mode (Clear Timer on Compare Match)
    // - Prescaler 64
    // - OCR1A = 15624

    tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));

    tmr1.tccr1b
        .write(|w| w.cs1().prescale_64().wgm1().bits(0b01));

    // Compare Interrupt enable
    // TIMSK1 |= (1<<OCIE1A);
    tmr1.timsk1.write(|w| w.ocie1a().set_bit());
}

pub fn set_duration(duration: Microseconds, tmr1: &TC1) {
    let time = *duration.integer() as u16;
    let compare_value = time / 4;
    tmr1.ocr1a.write(|w| unsafe { w.bits(compare_value) });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    use crate::STEP_PIN;
    avr_device::interrupt::free(|cs| {
        if let Some(ref mut pin) = STEP_PIN.borrow(cs).borrow_mut().deref_mut() {
            pin.toggle().ok();
        }
    });
}
