

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int price [ ], int n ) {
  int val [ n + 1 ];
  val [ 0 ] = 0;
  int i, j;
  for ( i = 1;
  i <= n;
  i ++ ) {
    int max_val = INT_MIN;
    for ( j = 0;
    j < i;
    j ++ ) max_val = max ( max_val, price [ j ] + val [ i - j - 1 ] );
    val [ i ] = max_val;
  }
  return val [ n ];
}


