

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int X ) {
  if ( X < arr [ 0 ] ) return 0;
  else if ( X > arr [ n - 1 ] ) return n;
  int lowerPnt = 0;
  int i = 1;
  while ( i < n && arr [ i ] < X ) {
    lowerPnt = i;
    i = i * 2;
  }
  while ( lowerPnt < n && arr [ lowerPnt ] < X ) lowerPnt ++;
  return lowerPnt;
}


