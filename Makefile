BUILD_DIR=Build
CC=avr-gcc
LD=avr-ld
LIBS=
STANDART=c99
OPTIMIZE=-Os
TARGET=base
MCU=atmega128

APPS=$(wildcard *.app)

C_SOURCES=$(wildcard $(addsuffix /*.c, $(APPS)))
C_HEADERS=$(wildcard $(addsuffix /*.h, $(APPS)))

C_INCLUDES=$(addprefix -I, $(APPS))

C_DEFS=-DF_CPU=16000000L

CFLAGS=$(C_DEFS) $(C_INCLUDES) $(OPTIMIZE) --std=$(STANDART) -Wno-write-strings -Wcast-align -Wcast-qual -Wconversion -Wduplicated-branches -Wduplicated-cond -Wfloat-equal -Wlogical-op -Wredundant-decls -Wsign-conversion -Werror -Werror -Wall -Wextra -Wpedantic -pedantic-errors -pedantic -mmcu=$(MCU)
LFLAGS=$(OPTIMIZE) -Wno-write-strings -Wcast-align -Wcast-qual -Wconversion -Wctor-dtor-privacy -Wduplicated-branches -Wduplicated-cond -Wextra-semi -Wfloat-equal -Wlogical-op -Wnon-virtual-dtor -Wold-style-cast -Woverloaded-virtual -Wredundant-decls -Wsign-conversion -Wsign-promo -Wall -Wextra -Wpedantic -pedantic-errors -flto -fuse-linker-plugin -ffunction-sections -fdata-sections -Wl,--gc-sections -mmcu=$(MCU) -lm $(LIBS)

OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))
vpath %.h $(sort $(dir $(C_HEADERS)))


all: date config size Dir $(C_HEADERS) codecheck.sh
	@echo $(C_SOURCES)

$(BUILD_DIR)/%.o: %.c Makefile | $(BUILD_DIR)
	@echo -e '\033[1;32mCC\t'$<'\t->\t'$@'\033[0m'
	@$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=$(BUILD_DIR)/$(notdir $(<:.c=.lst)) $< -o $@
	@avr-objdump -d -S $@ > $(BUILD_DIR)/$(notdir $(<:.c=.casm))


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


BuildDir:
	@mkdir -p $(BUILD_DIR)

Dir: BuildDir


.PHONY: clean
clean:
	@rm -rf $(BUILD_DIR)/*
	@echo -e '\033[0;31mCleaned\033[0m'

.NOTPARALLEL: date config
date:
	@echo -e '\033[1;32m'"Starting build at " | tr -d '\n'
	@date
	@echo -e '\033[0m'

config:
	@./config.sh
	@echo

.PHONY: codecheck.sh
codecheck.sh: $(C_SOURCES)
	@./codecheck.sh
