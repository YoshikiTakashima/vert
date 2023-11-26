

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr1 [ ], int arr2 [ ], int n1, int n2 ) {
  sort ( arr1, arr1 + n1 );
  sort ( arr2, arr2 + n2 );
  return arr1 [ n1 - 1 ] * arr2 [ 0 ];
}


