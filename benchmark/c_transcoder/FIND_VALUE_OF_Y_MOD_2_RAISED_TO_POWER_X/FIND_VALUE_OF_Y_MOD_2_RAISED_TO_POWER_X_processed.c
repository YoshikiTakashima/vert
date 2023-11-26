

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( long int y, long int x ) {
  if ( log2 ( y ) < x ) return y;
  if ( x > 63 ) return y;
  return ( y % ( 1 << x ) );
}


