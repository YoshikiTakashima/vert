

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int low, int high, int key ) {
  if ( high < low ) return - 1;
  int mid = ( low + high ) / 2;
  if ( key == arr [ mid ] ) return mid;
  if ( key > arr [ mid ] ) return f_gold ( arr, ( mid + 1 ), high, key );
  return f_gold ( arr, low, ( mid - 1 ), key );
}


