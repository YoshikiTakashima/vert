

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, int y ) {
  int res1 = log ( y ) / log ( x );
  double res2 = log ( y ) / log ( x );
  return ( res1 == res2 );
}


