

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int k ) {
  if ( k <= 0 ) return n;
  return ( n & ~ ( 1 << ( k - 1 ) ) );
}


