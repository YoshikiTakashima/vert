

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



long long f_gold ( int n ) {
  if ( n < 0 ) return 0;
  if ( n <= 1 ) return 1;
  double x = ( ( n * log10 ( n / M_E ) + log10 ( 2 * M_PI * n ) / 2.0 ) );
  return floor ( x ) + 1;
}


