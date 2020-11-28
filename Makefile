BUILD_DIR=Build
CC=avr-gcc
LD=avr-ld
LIBS=
STANDART=c99
OPTIMIZE=-Os
TARGET=base
MCU=atmega128
F_CPU=16000000L

APPS=$(wildcard *.app)

APP_OBJECTS=$(addsuffix /app.o, $(APPS))

CLEAN_OBJECTS=$(addprefix clean_, $(APPS))

all: date apps

.PHONY: apps clean
apps: $(APPS)

%.app: Makefile .FORCE
	@echo -e '\033[1;32mBuilding app '$@'\033[0m'
	@$(MAKE) -C $(shell echo $@ | sed 's/^clean_//g')

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