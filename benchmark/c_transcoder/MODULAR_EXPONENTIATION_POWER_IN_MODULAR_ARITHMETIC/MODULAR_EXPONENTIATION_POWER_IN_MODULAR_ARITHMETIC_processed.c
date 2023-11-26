

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, unsigned int y, int p ) {
  int res = 1;
  x = x % p;
  while ( y > 0 ) {
    if ( y & 1 ) res = ( res * x ) % p;
    y = y >> 1;
    x = ( x * x ) % p;
  }
  return res;
}


