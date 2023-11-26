

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int array [ ], int start, int end ) {
  if ( start > end ) return end + 1;
  if ( start != array [ start ] ) return start;
  int mid = ( start + end ) / 2;
  if ( array [ mid ] == mid ) return f_gold ( array, mid + 1, end );
  return f_gold ( array, start, mid );
}


