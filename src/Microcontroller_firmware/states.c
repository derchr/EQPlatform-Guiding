#include "globals.h"

#include <avr/io.h>
#include <string.h>
#include <stdlib.h>

unsigned int uart_us;

void track_state(void) {
    // Keep timer interrupt enabled.
    TIMSK1 |= (1<<OCIE1A);
}

void hold_state(void) {
    // Keep timer interrupt disabled.
    TIMSK1 &= ~(1<<OCIE1A);
}

void fast_state(void) {
    // Use fast speed.
    wait_us = 150;

    // Keep timer interrupt enabled.
    TIMSK1 |= (1<<OCIE1A);
}

void uart_state(void) {
    // We need to use strol because atoi can have undefined behaviour.
    uart_us = (int) strtol((char *) uart_string, NULL, 10);
    char opcode = -1;
    if (!uart_us) {
        // No numeric value sent.
        if (!strcmp((char *) uart_string, HOLD)) {
            opcode = hold;
        } else if (!strcmp((char *) uart_string, FAST_PLUS)) {
            opcode = fast_plus;
        } else if (!strcmp((char *) uart_string, FAST_MINUS)) {
            opcode = fast_minus;
        } else if (!strcmp((char *) uart_string, STATUS)) {
            opcode = status;
        } else if (!strcmp((char *) uart_string, REQUEST)) {
            opcode = request;
        } else if (uart_string[2] == '+') {
            opcode = ra_plus;
        } else if (uart_string[2] == '-') {
            opcode = ra_minus;
        }
    } else {
        // Numeric value sent.
        eeprom_write_us(uart_us);
        wait_us = uart_us;
    }
    processOPcode(opcode);
}
