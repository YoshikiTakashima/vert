

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, int y ) {
  x = x % 10;
  if ( y != 0 ) y = y % 4 + 4;
  return ( ( ( int ) ( pow ( x, y ) ) ) % 10 );
}


