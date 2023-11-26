

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n, int k ) {
  int p = 1;
  if ( k % 2 ) p = - 1;
  return ( pow ( n - 1, k ) + p * ( n - 1 ) ) / n;
}


