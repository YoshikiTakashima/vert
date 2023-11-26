

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int m, int n ) {
  if ( m < n ) return 0;
  if ( n == 0 ) return 1;
  return f_gold ( m - 1, n ) + f_gold ( m / 2, n - 1 );
}


