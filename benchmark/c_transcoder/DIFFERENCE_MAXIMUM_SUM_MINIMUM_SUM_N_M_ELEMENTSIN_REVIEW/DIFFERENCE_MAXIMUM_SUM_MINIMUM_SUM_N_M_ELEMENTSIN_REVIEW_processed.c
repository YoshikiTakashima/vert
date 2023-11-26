

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int m ) {
  int max = 0, min = 0;
  sort ( arr, arr + n );
  for ( int i = 0, j = n - 1;
  i < m;
  i ++, j -- ) {
    min += arr [ i ];
    max += arr [ j ];
  }
  return ( max - min );
}


