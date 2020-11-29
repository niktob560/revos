#ifndef __GPIO_C_H__
#define __GPIO_C_H__

#include <stdint.h>
#include <avr/io.h>

typedef enum 	{ OUTPUT, INPUT, INPUT_PULLUP }		GPIOMode;
typedef enum	{ HIGH = 1, LOW = 0, TOGGLE = 2 }	GPIOState;


#define __GPIODirReg__(port) (port - 1)
#define __GPIOPinReg__(port) (port - 2)

typedef struct
{
	const uint8_t 	pin;
	const GPIOState state;
} GPIOPinState;

inline static void GPIOSetPin(volatile uint8_t* port, const uint8_t pin, const GPIOState state) 
{
	if (state != TOGGLE)
	{
		*port = (uint8_t)(*port & ~(1 << pin)) | (uint8_t)(state << pin);
	}
	else
	{
		*port ^= (uint8_t)(1 << pin);
	}
}

inline static void GPIOSetState(volatile uint8_t* port, const GPIOPinState state) 
{
	GPIOSetPin(port, state.pin, state.state);
}

inline static void GPIOSetPinMde(volatile uint8_t* port, const uint8_t pin, const GPIOMode mode)
{
	volatile uint8_t *dir = __GPIODirReg__(port);
	switch (mode)
	{
		case OUTPUT:
		{
			*dir = (uint8_t)(((*dir) & ~(1 << pin)) | (1 << pin));
			break;
		}
		case INPUT_PULLUP:
		{
			GPIOSetPin(port, pin, HIGH);
			__attribute__((fallthrough));
		}
		case INPUT:
		{
			*dir = (uint8_t)((*dir) & ~(1 << pin));
			break;
		}
	}
}

inline static GPIOState getState(const volatile uint8_t *port, const uint8_t pin)
{
	return (*__GPIOPinReg__(port) >> pin) & 1;
}

#endif