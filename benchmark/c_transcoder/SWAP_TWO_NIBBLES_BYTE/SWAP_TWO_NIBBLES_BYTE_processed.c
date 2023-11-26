

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x ) {
  return ( ( x & 0x0F ) << 4 | ( x & 0xF0 ) >> 4 );
}


