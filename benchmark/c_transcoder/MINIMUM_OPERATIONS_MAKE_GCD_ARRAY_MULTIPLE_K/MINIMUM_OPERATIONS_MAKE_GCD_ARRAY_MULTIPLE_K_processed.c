

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n, int k ) {
  int result = 0;
  for ( int i = 0;
  i < n;
  ++ i ) {
    if ( a [ i ] != 1 && a [ i ] > k ) {
      result = result + min ( a [ i ] % k, k - a [ i ] % k );
    }
    else {
      result = result + k - a [ i ];
    }
  }
  return result;
}


