

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  sort ( arr, arr + n );
  int minXor = INT_MAX;
  int val = 0;
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    val = arr [ i ] ^ arr [ i + 1 ];
    minXor = min ( minXor, val );
  }
  return minXor;
}


