#include <stdint.h>
#include <stdio.h>
#include "base.h"
#include <avr/interrupt.h>


void (*_taskFunctions[])(void) = {__TASKS__};

const uint8_t _funcsCount = sizeof(_taskFunctions) / sizeof(_taskFunctions[0]);

volatile uint8_t _enabledBits[((sizeof(_taskFunctions) / sizeof(_taskFunctions[0])) / 8) + ((sizeof(_taskFunctions) / sizeof(_taskFunctions[0])) % 8 > 0? 1 : 0)] = {0};

volatile uint8_t _currentIndex = 0;

bool isTaskRunning(uint8_t id)
{
	uint8_t byte = id / 8,
	shift = id % 8;
	return (_enabledBits[byte] & (1 << shift)) != 0;
}

void setTaskState(uint8_t id, bool isRunning) 
{
	uint8_t byte = id / 8,
	shift = id % 8;
	_enabledBits[byte] = (uint8_t)((_enabledBits[byte] & ~(1 << shift)) | ((isRunning? 1 : 0) << shift));
}

void schedule() 
{
	cli();
	if (_taskFunctions[_currentIndex] != 0x00 && !isTaskRunning(_currentIndex))
	{
		setTaskState(_currentIndex, true);
		sei();
		_taskFunctions[_currentIndex]();
		cli();
		setTaskState(_currentIndex, false);
	}
	if (_currentIndex + 1 >= _funcsCount)
	{
		_currentIndex = 0;
	}
	else
	{
		_currentIndex = _currentIndex + 1;
	}
	sei();
}

#ifdef TCCR0
#define TCCR TCCR0
#define OCIE OCIE0
#define OCF OCF0
#define OCR OCR0
#elif defined(TCCR0A)
#define TCCR TCCR0A
#define OCIE OCIE0A
#define OCF OCF0A
#define OCR OCR0A
#define TIMSK TIMSK0
#define TIMER0_COMP_vect TIMER0_COMPA_vect
#else
#pragma error Unable to run schedule on this MCU
#endif

#ifndef TIFR
#ifdef EIFR
#define TIFR EIFR
#else
#pragma error Unable to run schedule on this MCU
#endif
#endif


int16_t main() 
{
	TCCR = (1 << WGM01) | (1 << CS02) | (0 << CS01) | (0 << CS00);
	TIMSK = 1 << OCIE;
	TIFR = 1 << OCF;
	OCR = 125;
	while(true) 
	{
		sei();
	}
	return 0;
}

ISR(TIMER0_COMP_vect) 
{
	schedule();
}