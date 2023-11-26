

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int p ) {
  n = n % p;
  for ( int x = 2;
  x < p;
  x ++ ) if ( ( x * x ) % p == n ) return 1;
  return 0;
}


