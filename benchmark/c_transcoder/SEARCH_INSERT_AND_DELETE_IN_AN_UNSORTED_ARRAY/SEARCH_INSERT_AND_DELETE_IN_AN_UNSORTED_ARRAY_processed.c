

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int key ) {
  int i;
  for ( i = 0;
  i < n;
  i ++ ) if ( arr [ i ] == key ) return i;
  return - 1;
}


