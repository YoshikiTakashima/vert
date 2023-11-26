

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int arr_size ) {
  for ( int i = 0;
  i < arr_size;
  i ++ ) {
    int count = 0;
    for ( int j = 0;
    j < arr_size;
    j ++ ) {
      if ( arr [ i ] == arr [ j ] ) count ++;
    }
    if ( count % 2 != 0 ) return arr [ i ];
  }
  return - 1;
}


