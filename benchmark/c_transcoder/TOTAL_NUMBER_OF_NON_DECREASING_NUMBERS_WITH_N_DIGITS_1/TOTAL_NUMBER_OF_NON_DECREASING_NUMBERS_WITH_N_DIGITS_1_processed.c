

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int N = 10;
  long long count = 1;
  for ( int i = 1;
  i <= n;
  i ++ ) {
    count *= ( N + i - 1 );
    count /= i;
  }
  return count;
}


