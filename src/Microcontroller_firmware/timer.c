#include "globals.h"

#include <avr/io.h>
#include <avr/interrupt.h>

void initTimer (void) {
    // Prescaler 64, CTC enable
    TCCR1B |= (1<<CS10) | (1<<CS11) | (1<<WGM12);

    // Compare Interrupt enable
    TIMSK1 |= (1<<OCIE1A);
}

void setCompRegister(uint16_t us) {
    // Set the Compare Register to trigger the timer1 interrupt when a certain timer value is reached.
    // This function does only work up to 262ms!
    uint16_t compareVal = 0.25 * us;
    OCR1A = compareVal;
}

ISR(TIMER1_COMPA_vect) {
    // This routine is triggered when the timer counter has reached the same value that is stored in the comparison register.
    PORTB ^= (1<<STEP_PIN);

    if (PORTB & (1<<STEP_PIN)) {
        // Is STEP_PIN HIGH
        setCompRegister(25);
    } else {
        // Is STEP_PIN LOW
        setCompRegister(wait_us - 25);
    }
}
