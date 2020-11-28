#ifndef __BASE_H__
    #define __BASE_H__
#include <stdint.h>
#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include <stdlib.h>


    #ifndef bool

typedef enum
{
	true = 1, 
	false = 0,
} bool;

    #endif /*ifndef bool*/


#define __TASKS__ main

extern void (*_taskFunctions[])();

#endif /*ifndef __BASE_H__*/