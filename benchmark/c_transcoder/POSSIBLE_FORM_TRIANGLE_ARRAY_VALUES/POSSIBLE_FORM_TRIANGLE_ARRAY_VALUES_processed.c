

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int N ) {
  if ( N < 3 ) return 0;
  sort ( arr, arr + N );
  for ( int i = 0;
  i < N - 2;
  i ++ ) if ( arr [ i ] + arr [ i + 1 ] > arr [ i + 2 ] ) return 1;
}


