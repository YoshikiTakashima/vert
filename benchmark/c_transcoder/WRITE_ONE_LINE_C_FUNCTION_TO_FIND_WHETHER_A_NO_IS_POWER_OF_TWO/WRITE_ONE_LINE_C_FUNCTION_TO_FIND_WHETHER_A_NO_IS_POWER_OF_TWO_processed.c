

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n == 0 ) return 0;
  while ( n != 1 ) {
    if ( n % 2 != 0 ) return 0;
    n = n / 2;
  }
  return 1;
}


