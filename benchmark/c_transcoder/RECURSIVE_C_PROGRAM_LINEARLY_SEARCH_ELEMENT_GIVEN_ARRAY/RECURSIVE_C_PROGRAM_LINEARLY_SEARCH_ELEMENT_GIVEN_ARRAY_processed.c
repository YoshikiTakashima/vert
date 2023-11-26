

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r < l ) return - 1;
  if ( arr [ l ] == x ) return l;
  if ( arr [ r ] == x ) return r;
  return f_gold ( arr, l + 1, r - 1, x );
}


