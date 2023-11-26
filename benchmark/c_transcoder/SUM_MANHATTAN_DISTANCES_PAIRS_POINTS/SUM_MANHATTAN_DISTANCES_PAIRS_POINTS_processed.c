

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x [ ], int y [ ], int n ) {
  int sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = i + 1;
  j < n;
  j ++ ) sum += ( abs ( x [ i ] - x [ j ] ) + abs ( y [ i ] - y [ j ] ) );
  return sum;
}


