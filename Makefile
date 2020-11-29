BUILD_DIR=Build
CC=avr-gcc
LD=avr-ld
LIBS=
STANDART=c99
OPTIMIZE=-Os
TARGET=main
MCU=atmega128
F_CPU=16000000L

APPS=$(wildcard *.app)

APP_OBJECTS=$(addsuffix /app.o, $(APPS))

CLEAN_OBJECTS=$(addprefix clean_, $(APPS))

LFLAGS=$(OPTIMIZE) -Wno-write-strings -Wcast-align -Wcast-qual -Wconversion -Wctor-dtor-privacy -Wduplicated-branches -Wduplicated-cond -Wextra-semi -Wfloat-equal -Wlogical-op -Wnon-virtual-dtor -Wold-style-cast -Woverloaded-virtual -Wredundant-decls -Wsign-conversion -Wsign-promo -Wall -Wextra -Wpedantic -pedantic-errors -flto -fuse-linker-plugin -ffunction-sections -fdata-sections -Wl,--gc-sections -mmcu=$(MCU) -lm $(LIBS)

# TODO: add drivers objects
OBJECTS=$(APP_OBJECTS)

all: date apps size

.PHONY: apps clean
apps: $(APPS)

%.app: Makefile .FORCE
	@echo -e '\033[1;32mBuilding app '$@'\033[0m'
	@$(MAKE) -C $(shell echo $@ | sed 's/^clean_//g')


$(BUILD_DIR)/$(TARGET).elf: $(OBJECTS)
	@echo -e '\033[1;32mELF\t'$(OBJECTS)' -> '$@'\033[0m'
	@$(CC) $(LFLAGS) $(OBJECTS) -o $(BUILD_DIR)/$(TARGET).elf

$(BUILD_DIR)/$(TARGET).hex: $(BUILD_DIR)/$(TARGET).elf
	@echo -e '\033[1;32mHEX\t'$<'\t->\t'$@'\033[0m'
	@avr-objcopy -O ihex -j .eeprom --set-section-flags=.eeprom=alloc,load --no-change-warnings --change-section-lma .eeprom=0  "$(BUILD_DIR)/$(TARGET).elf" "$(BUILD_DIR)/$(TARGET).eep"
	@avr-objcopy -O ihex -R .eeprom  "$(BUILD_DIR)/$(TARGET).elf" "$(BUILD_DIR)/$(TARGET).hex"
	@avr-objdump -d -S $(BUILD_DIR)/$(TARGET).elf > $(BUILD_DIR)/$(TARGET)_elf.casm

size: $(BUILD_DIR)/$(TARGET).hex
	@echo -e '\033[0;36m'
	@avr-size $(BUILD_DIR)/$(TARGET).elf -C --mcu=$(MCU)
	@echo -e '\033[0m'

clean_%.app:
	@echo -e '\033[0;31mCleaning app '$(shell echo $@ | sed 's/^clean_//g')'\033[0m'
	@$(MAKE) -C $(shell echo $@ | sed 's/^clean_//g') clean

clean: $(CLEAN_OBJECTS)
	@rm -rf $(BUILD_DIR)
	@echo -e '\033[0;31mCleaned\033[0m'

.FORCE:

.NOTPARALLEL: date
date:
	@echo -e '\033[1;32m'"Starting build at " | tr -d '\n'
	@date
	@echo -e '\033[0m'