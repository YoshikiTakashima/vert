

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b ) {
  if ( a == 0 || b == 0 ) return 1;
  return floor ( log10 ( abs ( a ) ) + log10 ( abs ( b ) ) ) + 1;
}


