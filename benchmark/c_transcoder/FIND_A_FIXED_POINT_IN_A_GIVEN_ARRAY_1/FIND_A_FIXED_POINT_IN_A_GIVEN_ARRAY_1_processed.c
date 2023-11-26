

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int low, int high ) {
  if ( high >= low ) {
    int mid = ( low + high ) / 2;
    if ( mid == arr [ mid ] ) return mid;
    if ( mid > arr [ mid ] ) return f_gold ( arr, ( mid + 1 ), high );
    else return f_gold ( arr, low, ( mid - 1 ) );
  }
  return - 1;
}


