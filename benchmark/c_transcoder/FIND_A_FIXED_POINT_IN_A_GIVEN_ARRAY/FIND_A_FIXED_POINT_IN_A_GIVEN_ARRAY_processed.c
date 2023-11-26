

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int i;
  for ( i = 0;
  i < n;
  i ++ ) {
    if ( arr [ i ] == i ) return i;
  }
  return - 1;
}


