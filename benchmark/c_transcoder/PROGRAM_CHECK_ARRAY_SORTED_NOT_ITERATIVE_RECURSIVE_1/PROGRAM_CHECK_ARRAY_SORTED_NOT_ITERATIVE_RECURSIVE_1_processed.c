

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  if ( n == 0 || n == 1 ) return 1;
  for ( int i = 1;
  i < n;
  i ++ ) if ( arr [ i - 1 ] > arr [ i ] ) return 0;
  return 1;
}


