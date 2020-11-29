#ifndef __BASE_H__
    #define __BASE_H__
#include <stdint.h>
// #include <avr/io.h>
// #include <avr/interrupt.h>
// #include <util/delay.h>
// #include <stdlib.h>

#include "../Build/mods.h"


    #ifndef bool

typedef enum
{
	true = 1, 
	false = 0,
} bool;

    #endif /*ifndef bool*/


#ifndef __TASKS__
#pragma error __TASKS__ variable does not defined
#endif

extern void (*_taskFunctions[])();

#endif /*ifndef __BASE_H__*/