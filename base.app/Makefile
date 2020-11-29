CC=avr-gcc
STANDART=c99
OPTIMIZE=-Os


C_INCLUDES=-I./

C_DEFS=-DF_CPU=$(F_CPU)

CFLAGS=$(C_DEFS) $(C_INCLUDES) $(OPTIMIZE) --std=$(STANDART) -Wno-write-strings -Wcast-align -Wcast-qual -Wconversion -Wduplicated-branches -Wduplicated-cond -Wfloat-equal -Wlogical-op -Wredundant-decls -Wsign-conversion -Werror -Werror -Wall -Wextra -Wpedantic -pedantic-errors -pedantic -mmcu=$(MCU)


all: app.o

app.o:
	@echo -e '\033[1;32mCC\t'base.c'\t->\t'$@'\033[0m'
	@$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=base.lst base.c -o app.o

.PHONY: clean
clean:
	@rm -rf base.lst app.o
	@echo -e '\033[0;31mCleaned\033[0m'
