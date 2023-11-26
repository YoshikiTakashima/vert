

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int k ) {
  int count;
  for ( int i = 0;
  i < n;
  i ++ ) {
    count = 0;
    for ( int j = 0;
    j < n;
    j ++ ) {
      if ( arr [ j ] == arr [ i ] ) count ++;
      if ( count > 2 * k ) return 0;
    }
  }
  return 1;
}


