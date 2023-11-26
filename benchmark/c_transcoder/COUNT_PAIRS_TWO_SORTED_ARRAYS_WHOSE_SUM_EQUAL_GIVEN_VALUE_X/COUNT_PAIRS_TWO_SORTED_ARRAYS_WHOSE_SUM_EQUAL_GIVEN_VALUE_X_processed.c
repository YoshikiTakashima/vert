

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr1 [ ], int arr2 [ ], int m, int n, int x ) {
  int count = 0;
  for ( int i = 0;
  i < m;
  i ++ ) for ( int j = 0;
  j < n;
  j ++ ) if ( ( arr1 [ i ] + arr2 [ j ] ) == x ) count ++;
  return count;
}


