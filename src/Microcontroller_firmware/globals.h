#ifndef GLOBALS
#define GLOBALS

#ifndef __AVR_ATmega328P__
#define __AVR_ATmega328P__ // The microcontroller needs to be recognized to enable syntax highlighting.
#endif

#ifndef F_CPU
#define F_CPU 16000000UL
#endif

#include <avr/io.h>

/******************************
** Definitions
*******************************/

#define VERSION "0.9"

#define BAUD 57600

#define UART_MAXSTRLEN 10

// *** Pin definitions ***

#define STEP_PIN    PB0
#define DIR_PIN     PB5
#define M0_PIN      PB1
#define M1_PIN      PB2
#define M2_PIN      PB3

// *** UART incoming Commands ***

#define HOLD        "h"
#define TRACK       "0"
#define FAST_PLUS   "f+"
#define FAST_MINUS  "f-"
#define STATUS      "s"
#define REQUEST     "req"

// These are for pulse guiding:
#define RA_PLUS     "RA+"
#define RA_MINUS    "RA-"

/******************************
** Extern Variables
*******************************/

// This variable will be the relevant for the timer.
extern volatile unsigned int wait_us;

// This variable will be written by the UART connection or the EEPROM.
extern unsigned int uart_us;

extern volatile char uart_string[UART_MAXSTRLEN + 1];

extern uint16_t runtimes;

// Opcodes from uart:
enum uart_opcodes {hold, fast_minus, fast_plus, status, request, ra_minus, ra_plus};

/******************************
** Function Prototypes
*******************************/

// main.c
void (*curr_state)(void);
void processOPcode(char opcode);
void delayMicroseconds(int ms);

// states.c
void track_state(void);
void fast_state(void);
void hold_state(void);
void uart_state(void);

//timer.c
void initTimer(void);
void setCompRegister(uint16_t us);

// uart.c
void initUart(void);
void uart_putC(unsigned char charToSend);
void uart_putS(char *stringToSend);
void uart_putInt(int intToSend);
void uart_putUInt(uint16_t uintToSend);
void uart_sendStatus(void);
void uart_sendStatusH(void);

// eeprom.c
void incrementRuntime(void);
uint16_t eeprom_read_us(void);
void eeprom_write_us(uint16_t us);

#endif