

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int p ) {
  if ( n >= p ) return 0;
  int result = 1;
  for ( int i = 1;
  i <= n;
  i ++ ) result = ( result * i ) % p;
  return result;
}


