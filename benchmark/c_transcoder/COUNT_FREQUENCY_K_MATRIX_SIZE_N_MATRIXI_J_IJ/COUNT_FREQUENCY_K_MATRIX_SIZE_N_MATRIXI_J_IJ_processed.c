

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int k ) {
  if ( n + 1 >= k ) return ( k - 1 );
  else return ( 2 * n + 1 - k );
}


