

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int res = 1;
  for ( int i = 0;
  i < n && arr [ i ] <= res;
  i ++ ) res = res + arr [ i ];
  return res;
}


