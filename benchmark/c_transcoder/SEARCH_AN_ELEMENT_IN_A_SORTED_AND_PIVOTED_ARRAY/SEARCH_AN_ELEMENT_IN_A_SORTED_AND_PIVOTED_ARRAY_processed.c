

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int l, int h, int key ) {
  if ( l > h ) return - 1;
  int mid = ( l + h ) / 2;
  if ( arr [ mid ] == key ) return mid;
  if ( arr [ l ] <= arr [ mid ] ) {
    if ( key >= arr [ l ] && key <= arr [ mid ] ) return f_gold ( arr, l, mid - 1, key );
    return f_gold ( arr, mid + 1, h, key );
  }
  if ( key >= arr [ mid ] && key <= arr [ h ] ) return f_gold ( arr, mid + 1, h, key );
  return f_gold ( arr, l, mid - 1, key );
}


