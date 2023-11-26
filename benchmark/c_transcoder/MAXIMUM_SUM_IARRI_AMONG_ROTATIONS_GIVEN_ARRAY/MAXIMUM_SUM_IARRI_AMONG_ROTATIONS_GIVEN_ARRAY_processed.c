

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int res = INT_MIN;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int curr_sum = 0;
    for ( int j = 0;
    j < n;
    j ++ ) {
      int index = ( i + j ) % n;
      curr_sum += j * arr [ index ];
    }
    res = max ( res, curr_sum );
  }
  return res;
}


