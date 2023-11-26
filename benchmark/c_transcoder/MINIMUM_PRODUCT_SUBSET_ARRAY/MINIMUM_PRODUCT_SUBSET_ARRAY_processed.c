

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  if ( n == 1 ) return a [ 0 ];
  int max_neg = INT_MIN;
  int min_pos = INT_MAX;
  int count_neg = 0, count_zero = 0;
  int prod = 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( a [ i ] == 0 ) {
      count_zero ++;
      continue;
    }
    if ( a [ i ] < 0 ) {
      count_neg ++;
      max_neg = max ( max_neg, a [ i ] );
    }
    if ( a [ i ] > 0 ) min_pos = min ( min_pos, a [ i ] );
    prod = prod * a [ i ];
  }
  if ( count_zero == n || ( count_neg == 0 && count_zero > 0 ) ) return 0;
  if ( count_neg == 0 ) return min_pos;
  if ( ! ( count_neg & 1 ) && count_neg != 0 ) {
    prod = prod / max_neg;
  }
  return prod;
}


