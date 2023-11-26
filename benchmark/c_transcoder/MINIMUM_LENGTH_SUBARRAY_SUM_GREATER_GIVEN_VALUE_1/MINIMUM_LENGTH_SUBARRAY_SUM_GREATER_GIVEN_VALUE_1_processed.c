

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int x ) {
  int curr_sum = 0, min_len = n + 1;
  int start = 0, end = 0;
  while ( end < n ) {
    while ( curr_sum <= x && end < n ) {
      if ( curr_sum <= 0 && x > 0 ) {
        start = end;
        curr_sum = 0;
      }
      curr_sum += arr [ end ++ ];
    }
    while ( curr_sum > x && start < n ) {
      if ( end - start < min_len ) min_len = end - start;
      curr_sum -= arr [ start ++ ];
    }
  }
  return min_len;
}


