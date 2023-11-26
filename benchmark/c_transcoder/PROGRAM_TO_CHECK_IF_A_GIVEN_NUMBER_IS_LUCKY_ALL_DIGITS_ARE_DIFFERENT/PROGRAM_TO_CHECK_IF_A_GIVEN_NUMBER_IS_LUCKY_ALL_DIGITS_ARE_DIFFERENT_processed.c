

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  bool arr [ 10 ];
  for ( int i = 0;
  i < 10;
  i ++ ) arr [ i ] = 0;
  while ( n > 0 ) {
    int digit = n % 10;
    if ( arr [ digit ] ) return 0;
    arr [ digit ] = 1;
    n = n / 10;
  }
  return 1;
}


