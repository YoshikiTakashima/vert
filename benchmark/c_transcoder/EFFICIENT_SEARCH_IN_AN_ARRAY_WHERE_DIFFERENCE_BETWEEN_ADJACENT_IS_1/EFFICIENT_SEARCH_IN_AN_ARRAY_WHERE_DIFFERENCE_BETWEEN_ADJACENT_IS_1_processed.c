

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int x ) {
  int i = 0;
  while ( i <= n - 1 ) {
    if ( arr [ i ] == x ) return i;
    i += abs ( arr [ i ] - x );
  }
  return - 1;
}


