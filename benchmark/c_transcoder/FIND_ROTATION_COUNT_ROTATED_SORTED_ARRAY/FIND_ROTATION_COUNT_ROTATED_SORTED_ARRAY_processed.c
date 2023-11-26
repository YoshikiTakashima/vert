

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int min = arr [ 0 ], min_index;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( min > arr [ i ] ) {
      min = arr [ i ];
      min_index = i;
    }
  }
  return min_index;
}


