

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, int y ) {
  if ( x == 1 ) return ( y == 1 );
  int pow = 1;
  while ( pow < y ) pow *= x;
  return ( pow == y );
}


