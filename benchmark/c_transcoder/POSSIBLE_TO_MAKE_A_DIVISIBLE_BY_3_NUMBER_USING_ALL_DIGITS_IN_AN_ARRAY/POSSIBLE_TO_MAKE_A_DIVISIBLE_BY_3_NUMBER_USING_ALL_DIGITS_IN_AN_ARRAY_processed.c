

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int remainder = 0;
  for ( int i = 0;
  i < n;
  i ++ ) remainder = ( remainder + arr [ i ] ) % 3;
  return ( remainder == 0 );
}


