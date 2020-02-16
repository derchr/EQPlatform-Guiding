#include "globals.h"

#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/setbaud.h>
#include <stdlib.h>

volatile uint8_t uart_str_count = 0;
volatile char uart_string[UART_MAXSTRLEN + 1] = "";

void initUart (void) {
    // Set UART Baud Rate Register to the value calculated by the setbaud.h library.
    UBRR0 = UBRR_VALUE;

    // Set UART Control and Status Register B:
    // Enable RX Interrupt, enable RX, enable TX.
    UCSR0B |= (1<<RXCIE0) | (1<<RXEN0) | (1<<TXEN0);

    // Set UART Control and Status Register C:
    // Asynchronous 8N1
    UCSR0C |= (1<<UCSZ01) | (1<<UCSZ00);

    // USE_2X is 1 if the desired baud rate tolerance could only be achieved by setting the U2X bit in the UART configuration.
    #if USE_2X
    UCSR0A |= (1 << U2X0);
    #else
    UCSR0A &= ~(1 << U2X0);
    #endif
}

void uart_putC(unsigned char toSend) {
    // Wait until sending a character is possible.
    // Better implementation would be to make use of the FIFO Buffer.
    while (!(UCSR0A & (1<<UDRE0)));                     

    UDR0 = toSend;
}

void uart_putS(char *stringToSend) {
    while (*stringToSend) {
        uart_putC(*stringToSend);
        stringToSend++;
    }
}

void uart_putInt(int intToSend) {
    char string[8];
    itoa(intToSend, string, 10);
    uart_putS(string);
}

void uart_putUInt(uint16_t uintToSend) {
    char string[8];
    // ltoa because uintToSend could cause an overflow when using itoa.
    ltoa(uintToSend, string, 10);
    uart_putS(string);
}

void uart_sendStatus(void) {
    uart_putUInt(wait_us);
    uart_putC('#');
    uart_putUInt(runtimes);
    uart_putC('#');
}

void uart_sendStatusH(void) {
    uart_putS((char *) "~~~~~~~~~~ EQPlatform-PulseGuiding ~~~~~~~~~~\n\n");
    uart_putS((char *) "Firmware-Version: ");
    uart_putS(VERSION);
    uart_putC('\n');
    uart_putS((char *) "Current Velocity: ");
    uart_putUInt(wait_us);
    uart_putS((char *) "\n\n");
    uart_putS((char *) "Number of starts: ");
    uart_putUInt(runtimes);
    uart_putS((char *) "\n\n");
}

ISR (USART_RX_vect) {
    // Read UART Data buffer.
    unsigned char nextChar = UDR0;

    if (nextChar != '\r' && uart_str_count < UART_MAXSTRLEN) {
        uart_string[uart_str_count] = nextChar;
        uart_str_count++;
    } else {
        uart_string[uart_str_count] = '\0';
        uart_str_count = 0;
        curr_state = uart_state;
    }
}
