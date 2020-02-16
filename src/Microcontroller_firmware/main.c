/*  Copyright (C) 2020
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, see <http://www.gnu.org/licenses/>.
 */

#include "globals.h"

#include <stdlib.h>
#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>

volatile unsigned int wait_us;

/*  This program does only support guiding in one direction (RA, right ascension in this case),
    because the platform only has one stepper motor. */

/*  I noticed some problems when accelerating after receiving a guiding pulse. (The platform starts shaking.)
    Maybe I will add a smooth acceleration in the future. */

int main(void) {

    DDRB |= (1<<STEP_PIN) | (1<<DIR_PIN);
    DDRB |= (1<<M0_PIN) | (1<<M1_PIN) | (1<<M2_PIN);

    // 1/32 Steps.
    PORTB |= (1<<M0_PIN) | (1<<M1_PIN) | (1<<M2_PIN);

    initTimer();
    initUart();
    incrementRuntime();
    wait_us = eeprom_read_us();

    curr_state = track_state;

    // Global interrupt enable.
    sei();

    while(1) {
        // Execute current state.
        curr_state();
    }
    return 0;
}

void processOPcode(char opcode) {
    switch(opcode){
        case hold:
            curr_state = hold_state;
            break;

        case fast_plus:
            // Use positive direction.
            PORTB &= ~(1<<DIR_PIN);
            curr_state = fast_state;
            break;

        case fast_minus:
            // Use negative direction.
            PORTB |= (1<<DIR_PIN);
            curr_state = fast_state;
            break;

        case request:
            // Send informations.
            uart_sendStatus();
            curr_state = track_state;
            break;

        case status:
            // Send informations human readable.
            uart_sendStatusH();
            curr_state = track_state;
            break;

        case ra_plus:
            // Use positive direction and use faster speed.
            PORTB &= ~(1<<DIR_PIN);
            wait_us /= 2;
            delayMicroseconds(atoi((char *) &uart_string[3]));
            // Restore default mode.
            processOPcode(-1);
            break;

        case ra_minus:
            // Don't move, just disable timer interrupt.
            TIMSK1 &= ~(1<<OCIE1A);
            delayMicroseconds(atoi((char *) &uart_string[3]));
            // Restore default mode.
            processOPcode(-1);
            break;

        default:
            // 0 or undefined value sent (Start Tracking).
            wait_us = eeprom_read_us();
            uart_putUInt(wait_us);
            // Use normal direction.
            PORTB &= ~(1<<DIR_PIN);
            curr_state = track_state;
            break;
    }    
    uart_putS("\nOK#\n");
}

void delayMicroseconds(int ms) {
    // We need this function, because _delay_ms() needs its value at compile time.
    for(int i = 0; i<ms; i++) {
        _delay_ms(1);
    }
}
