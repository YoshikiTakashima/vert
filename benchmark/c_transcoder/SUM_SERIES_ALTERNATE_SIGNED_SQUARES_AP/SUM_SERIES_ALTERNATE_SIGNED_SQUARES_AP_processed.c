

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int a [ ] ) {
  int res = 0;
  for ( int i = 0;
  i < 2 * n;
  i ++ ) {
    if ( i % 2 == 0 ) res += a [ i ] * a [ i ];
    else res -= a [ i ] * a [ i ];
  }
  return res;
}


