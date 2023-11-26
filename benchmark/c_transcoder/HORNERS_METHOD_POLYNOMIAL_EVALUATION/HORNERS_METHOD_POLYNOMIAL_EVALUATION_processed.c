

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int poly [ ], int n, int x ) {
  int result = poly [ 0 ];
  for ( int i = 1;
  i < n;
  i ++ ) result = result * x + poly [ i ];
  return result;
}


