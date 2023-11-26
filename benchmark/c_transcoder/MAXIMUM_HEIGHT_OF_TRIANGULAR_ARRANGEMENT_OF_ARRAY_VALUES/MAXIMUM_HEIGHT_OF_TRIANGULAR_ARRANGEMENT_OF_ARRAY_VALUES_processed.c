

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  int result = 1;
  for ( int i = 1;
  i <= n;
  ++ i ) {
    long long y = ( i * ( i + 1 ) ) / 2;
    if ( y < n ) result = i;
    else break;
  }
  return result;
}


