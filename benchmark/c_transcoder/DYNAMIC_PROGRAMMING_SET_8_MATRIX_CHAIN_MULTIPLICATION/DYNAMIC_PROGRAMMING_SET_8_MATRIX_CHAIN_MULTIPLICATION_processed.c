

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int p [ ], int i, int j ) {
  if ( i == j ) return 0;
  int k;
  int min = INT_MAX;
  int count;
  for ( k = i;
  k < j;
  k ++ ) {
    count = f_gold ( p, i, k ) + f_gold ( p, k + 1, j ) + p [ i - 1 ] * p [ k ] * p [ j ];
    if ( count < min ) min = count;
  }
  return min;
}


