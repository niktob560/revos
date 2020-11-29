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

int16_t main() 
{
	TCCR0 = (1 << WGM01) | (1 << CS02) | (0 << CS01) | (0 << CS00);
	TIMSK = 1 << OCIE0;
	TIFR = 1 << OCF0;
	OCR0 = 125;
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