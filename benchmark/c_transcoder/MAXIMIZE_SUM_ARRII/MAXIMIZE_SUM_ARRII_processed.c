

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  sort ( arr, arr + n );
  int sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) sum += ( arr [ i ] * i );
  return sum;
}


