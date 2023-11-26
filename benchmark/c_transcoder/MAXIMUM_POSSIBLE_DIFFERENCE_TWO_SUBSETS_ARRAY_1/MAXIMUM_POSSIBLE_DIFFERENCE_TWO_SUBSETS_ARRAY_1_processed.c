

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int result = 0;
  sort ( arr, arr + n );
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    if ( arr [ i ] != arr [ i + 1 ] ) result += abs ( arr [ i ] );
    else i ++;
  }
  if ( arr [ n - 2 ] != arr [ n - 1 ] ) result += abs ( arr [ n - 1 ] );
  return result;
}


