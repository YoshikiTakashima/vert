

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int low, int high ) {
  int max = arr [ low ];
  int i;
  for ( i = low + 1;
  i <= high;
  i ++ ) {
    if ( arr [ i ] > max ) max = arr [ i ];
    else break;
  }
  return max;
}


