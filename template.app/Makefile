CC=avr-gcc
STANDART=c99
OPTIMIZE=-Os



C_INCLUDES=-I../

C_DEFS=-DF_CPU=$(F_CPU)

CFLAGS=$(C_DEFS) $(C_INCLUDES) $(OPTIMIZE) --std=$(STANDART) -Wno-write-strings -Wcast-align -Wcast-qual -Wconversion -Wduplicated-branches -Wduplicated-cond -Wfloat-equal -Wlogical-op -Wredundant-decls -Wsign-conversion -Werror -Werror -Wall -Wextra -Wpedantic -pedantic-errors -pedantic -mmcu=$(MCU)


all: app.o

app.o:
	@echo -e '\033[1;32mCC\t'main.c'\t->\t'$@'\033[0m'
	@$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=main.lst main.c -o app.o

.PHONY: clean
clean:
	@echo $(F_CPU)
	@rm -rf main.lst app.o
	@echo -e '\033[0;31mCleaned\033[0m'
