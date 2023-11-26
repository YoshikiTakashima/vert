

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n == 1 ) return 2;
  return 2 * f_gold ( n - 1 );
}


