

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int arr [ n ] [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = 0;
  j < n;
  j ++ ) arr [ i ] [ j ] = abs ( i - j );
  int sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = 0;
  j < n;
  j ++ ) sum += arr [ i ] [ j ];
  return sum;
}


