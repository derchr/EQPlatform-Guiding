#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

// ===========================================================================
// Modules
// ===========================================================================
mod eeprom;
mod serial;
mod state_machine;
mod timer;

// ===========================================================================
// Use declarations
// ===========================================================================
use core::cell::RefCell;

use atmega328p_hal as hal;
use avr_device::interrupt::Mutex;
use hal::clock::MHz16;
use hal::port::mode::*;
use hal::port::*;
use hal::prelude::*;
use hal::usart::*;

use embedded_time::duration::*;
use staticvec::StaticString;

use panic_halt as _;

use crate::serial::InputVariant;
use crate::state_machine::*;

// ===========================================================================
// Types
// ===========================================================================

/// Type definition for the USART0 reader
type Usart0Reader =
    UsartReader<hal::pac::USART0, portd::PD0<Input<Floating>>, portd::PD1<Output>, MHz16>;
type Usart0Writer =
    UsartWriter<hal::pac::USART0, portd::PD0<Input<Floating>>, portd::PD1<Output>, MHz16>;

// ===========================================================================
// Structs
// ===========================================================================

/// Serial Buffer object that holds the buffer and the receiver side of the usart.
struct SerialBuffer {
    usart0_rx: Usart0Reader,
    buffer: StaticString<64>,
    is_complete: bool,
}

/// Timer struct that hold the timer register (it has to be altered in an ISR)
/// and the corresponding timer pin which is conrtolled by the timer.
struct TimerStructure {
    pin: portb::PB1<Output>,
    pin_is_high: bool,
    tc1: hal::pac::TC1,
}

// ===========================================================================
// Global Variables
// ===========================================================================

/// The step pin controls the motor.
static TIMER_STRUCTURE: Mutex<RefCell<Option<TimerStructure>>> = Mutex::new(RefCell::new(None));
/// The USART buffer object that will be used in the ISR to store the received bytes.
static SERIAL_BUFFER: Mutex<RefCell<Option<SerialBuffer>>> = Mutex::new(RefCell::new(None));

#[atmega328p_hal::entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();
    let portd = dp.PORTD.split();

    let step_pin = portb.pb1.into_output(&mut portb.ddr);
    let mut dir_pin = portb.pb5.into_output(&mut portb.ddr);
    let tc1 = dp.TC1;

    // Initialize the serial communication
    let mut serial_handler = serial::SerialHandler::new(dp.USART0, portd);

    eeprom::increment_runtime();

    // Get the last waiting time from eeprom
    let waiting_time = eeprom::read_waiting_time();

    avr_device::interrupt::free(|cs| {
        TIMER_STRUCTURE.borrow(cs).replace(Some(TimerStructure {
            pin: step_pin,
            pin_is_high: false,
            tc1,
        }));
    });

    // Initialize timer
    timer::init();
    timer::set_duration(waiting_time);
    timer::set_timer_status(true);

    // SAFETY:
    // We are not in a critical section, so enabling interrupts is fine.
    unsafe {
        // Enable interrupts globally
        avr_device::interrupt::enable();
    }

    // Create the state machine
    let mut eq_tracker = state_machine::EQTracker::new(waiting_time);

    loop {
        let input = serial_handler.handle_input();

        match input {
            Some(InputVariant::Track) => {
                serial_handler.write_str("Track!\n");
                eq_tracker.set_state(State::Track);
                timer::set_duration(eq_tracker.get_waiting_time());
                timer::set_timer_status(true);
                dir_pin.set_high().void_unwrap();
            },
            Some(InputVariant::TrackNewTime(duration)) => {
                serial_handler.write_str("Track with new duration: ");
                serial_handler.write_number(*duration.integer());
                serial_handler.write_str("us\n");
                eq_tracker.set_state(State::Track);
                timer::set_duration(duration);
                timer::set_timer_status(true);
                dir_pin.set_high().void_unwrap();
            },
            Some(InputVariant::Hold) => {
                serial_handler.write_str("Hold Hold Hold!\n");
                eq_tracker.set_state(State::Hold);
                timer::set_timer_status(false);
            },
            Some(InputVariant::FastForward(direction)) => {
                serial_handler.write_str("Fast Forward Mode!\n");
                eq_tracker.set_state(State::FastForward(direction));
                timer::set_duration(Microseconds(100));
                timer::set_timer_status(true);
                if direction {
                    dir_pin.set_high().void_unwrap();
                } else {
                    dir_pin.set_low().void_unwrap();
                }
            },
            Some(InputVariant::SetDefault) => {
                serial_handler.write_str("Write Default Value!\n");
                // ... TODO
            },
            Some(InputVariant::Status) => {
                serial_handler.send_status(*eq_tracker.get_waiting_time().integer(), 12, 12)
            },
            Some(InputVariant::Invalid) => {
                serial_handler.write_str("Invalid operation!\n")
            },

            None => (),
        }
    }
}
