

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  sort ( arr, arr + n );
  return arr [ n - 1 ] + arr [ n - 2 ] + arr [ n - 3 ];
}


