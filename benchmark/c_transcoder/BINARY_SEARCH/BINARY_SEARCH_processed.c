

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int l, int r, int x ) {
  if ( r >= l ) {
    int mid = l + ( r - l ) / 2;
    if ( arr [ mid ] == x ) return mid;
    if ( arr [ mid ] > x ) return f_gold ( arr, l, mid - 1, x );
    return f_gold ( arr, mid + 1, r, x );
  }
  return - 1;
}


