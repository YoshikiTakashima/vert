

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  sort ( arr, arr + n );
  int a = 0, b = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i & 1 ) a = a * 10 + arr [ i ];
    else b = b * 10 + arr [ i ];
  }
  return a + b;
}


