

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr1 [ ], int arr2 [ ], int n ) {
  for ( int i = 0;
  i < n;
  i ++ ) if ( arr1 [ i ] != arr2 [ i ] ) return i;
  return n;
}


