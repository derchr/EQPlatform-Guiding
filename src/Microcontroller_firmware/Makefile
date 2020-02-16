# Makefile 

# This is our target microcontroller.
MCU = atmega328p

TARGET = main

SRC = $(TARGET).c timer.c uart.c eeprom.c states.c

# Programmer

AVRDUDE_PROGRAMMER = usbasp
AVRDUDE_PORT = usb

# By removing '#', the EEPROM will be flashed too.
# AVRDUDE_WRITE_EEPROM = -U eeprom:w:$(TARGET).eep

# Upload via bootloader
SERIAL_BAUD = 57600


# Usually nothing has to be changed down here:
OBJECTS=$(SRC:.c=.o)
CFLAGS=-c -Os -Wall -Wextra -Werror
LDFLAGS=

# all: hex eeprom
all: hex

hex: $(TARGET).hex

# eeprom: $(TARGET)_eeprom.hex

$(TARGET).hex: $(TARGET).elf
	avr-objcopy -O ihex -j .data -j .text $(TARGET).elf $(TARGET).hex

$(TARGET)_eeprom.hex: $(TARGET).elf
	avr-objcopy -O ihex -j .eeprom --change-section-lma .eeprom=1 $(TARGET).elf $(TARGET)_eeprom.hex

$(TARGET).elf: $(OBJECTS)
	avr-gcc $(LDFLAGS) -mmcu=$(MCU) $(OBJECTS) -o $(TARGET).elf

.c.o:
	avr-gcc $(CFLAGS) -mmcu=$(MCU) $< -o $@

size:
	avr-size --mcu=$(MCU) -C $(TARGET).elf

program:
	avrdude -p $(MCU) $(AVRDUDE_PORT) $(BAUD) -c $(AVRDUDE_PROGRAMMER) -U flash:w:$(TARGET).hex:a

upload:
	avr-fboot -d /dev/rfcomm0 -b $(SERIAL_BAUD) -P Peda -p main.hex

clean_tmp:
	rm -rf *.o
	rm -rf *.elf

clean:
	rm -rf *.o
	rm -rf *.elf
	rm -rf *.hex