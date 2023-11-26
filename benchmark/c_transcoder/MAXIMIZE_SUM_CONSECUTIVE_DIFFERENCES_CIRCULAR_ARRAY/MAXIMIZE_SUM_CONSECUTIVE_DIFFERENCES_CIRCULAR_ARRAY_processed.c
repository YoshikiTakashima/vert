

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int sum = 0;
  sort ( arr, arr + n );
  for ( int i = 0;
  i < n / 2;
  i ++ ) {
    sum -= ( 2 * arr [ i ] );
    sum += ( 2 * arr [ n - i - 1 ] );
  }
  return sum;
}


