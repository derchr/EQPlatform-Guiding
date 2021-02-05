#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

// ===========================================================================
// Modules
// ===========================================================================
//mod state_machine;
mod eeprom;
mod serial;
mod timer;

// ===========================================================================
// Use declarations
// ===========================================================================
use panic_halt as _;

use core::cell::RefCell;

use atmega328p_hal as hal;
use hal::prelude::*;
use hal::clock::MHz16;
use hal::usart::*;
use hal::port::mode::*;
use hal::port::*;

use avr_device::interrupt::Mutex;

use arrayvec::ArrayString;

// ===========================================================================
// Types
// ===========================================================================

/// Type definition for the USART0 reader
type Usart0Reader = UsartReader<hal::pac::USART0, portd::PD0<Input<Floating>>, portd::PD1<Output>, MHz16>;
type Usart0Writer = UsartWriter<hal::pac::USART0, portd::PD0<Input<Floating>>, portd::PD1<Output>, MHz16>;

enum InputVariant {
    Track(embedded_time::duration::Microseconds),
    Hold,
    FastForward(bool),
    SetDefault,
    Status,
}

// ===========================================================================
// Structs
// ===========================================================================

/// Serial Buffer object that holds the buffer and the receiver side of the usart.
struct SerialBuffer {
    usart0_rx: Usart0Reader,
    buffer: ArrayString<[u8; 64]>,
    is_complete: bool, 
}

// ===========================================================================
// Global Variables
// ===========================================================================

/// The step pin controls the motor. 
static STEP_PIN: Mutex<RefCell<Option<portb::PB1<Output>>>> = Mutex::new(RefCell::new(None));
/// The USART buffer object that will be used in the ISR to store the received bytes.
static SERIAL_BUFFER: Mutex<RefCell<Option<SerialBuffer>>> = Mutex::new(RefCell::new(None));

#[atmega328p_hal::entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();
    let portd = dp.PORTD.split();

    let pb1 = portb.pb1.into_output(&mut portb.ddr);
    
    // Initialize the serial communication
    let mut serial_handler = serial::SerialHandler::new(dp.USART0, portd);

    avr_device::interrupt::free(|cs| {
        STEP_PIN.borrow(cs).replace(Some(pb1));
    });

    eeprom::increment_runtime();

    // Get the last waiting time from eeprom
    let waiting_time = eeprom::read_waiting_time();

    // Initialize timer
    timer::init(&dp.TC1);
    timer::set_duration(waiting_time, &dp.TC1);

    // SAFETY:
    // We are not in a critical section, so enabling interrupts is fine.
    unsafe {
        // Enable interrupts globally
        avr_device::interrupt::enable();
    }

    serial_handler.send_status();

    loop {
        serial_handler.handle_input();
    }
}
