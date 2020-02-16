#include "globals.h"

#include <avr/eeprom.h>

/*  The EEPROM will hold the last velocity set by UART,
    so it can be automatically loaded on startup.
    It also holds the number of runtimes of the program. */

uint16_t runtimes;
uint16_t * pointer = 0x00;

void incrementRuntime(void) {
	runtimes = eeprom_read_word(pointer);
	runtimes++;
	eeprom_write_word(pointer, runtimes);
}

uint16_t eeprom_read_us(void) {
	return eeprom_read_word(pointer+1);
}

void eeprom_write_us(uint16_t us) {
	eeprom_write_word(pointer+1, us);
}
