

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  if ( n == 1 || n == 0 ) return 1;
  if ( arr [ n - 1 ] < arr [ n - 2 ] ) return 0;
  return f_gold ( arr, n - 1 );
}


