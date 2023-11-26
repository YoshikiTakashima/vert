

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  if ( n == 1 ) return 1;
  sort ( arr, arr + n );
  int d = arr [ 1 ] - arr [ 0 ];
  for ( int i = 2;
  i < n;
  i ++ ) if ( arr [ i ] - arr [ i - 1 ] != d ) return 0;
  return 1;
}


