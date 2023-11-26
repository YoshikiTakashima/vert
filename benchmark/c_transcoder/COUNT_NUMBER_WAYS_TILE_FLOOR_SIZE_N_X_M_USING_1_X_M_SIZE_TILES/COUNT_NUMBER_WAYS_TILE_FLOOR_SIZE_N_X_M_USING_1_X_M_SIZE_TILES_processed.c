

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int m ) {
  int count [ n + 1 ];
  count [ 0 ] = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) {
    if ( i > m ) count [ i ] = count [ i - 1 ] + count [ i - m ];
    else if ( i < m ) count [ i ] = 1;
    else count [ i ] = 2;
  }
  return count [ n ];
}


