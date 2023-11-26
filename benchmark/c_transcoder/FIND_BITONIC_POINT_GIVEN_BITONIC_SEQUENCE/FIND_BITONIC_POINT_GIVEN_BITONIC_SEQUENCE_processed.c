

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int left, int right ) {
  if ( left <= right ) {
    int mid = ( left + right ) / 2;
    if ( arr [ mid - 1 ] < arr [ mid ] && arr [ mid ] > arr [ mid + 1 ] ) return mid;
    if ( arr [ mid ] < arr [ mid + 1 ] ) return f_gold ( arr, mid + 1, right );
    else return f_gold ( arr, left, mid - 1 );
  }
  return - 1;
}


