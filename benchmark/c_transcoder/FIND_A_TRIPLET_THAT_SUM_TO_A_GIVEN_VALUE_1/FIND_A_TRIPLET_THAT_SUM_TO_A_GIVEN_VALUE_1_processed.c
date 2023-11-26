

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int A [ ], int arr_size, int sum ) {
  int l, r;
  sort ( A, A + arr_size );
  for ( int i = 0;
  i < arr_size - 2;
  i ++ ) {
    l = i + 1;
    r = arr_size - 1;
    while ( l < r ) {
      if ( A [ i ] + A [ l ] + A [ r ] == sum ) {
        printf ( "Triplet is %d, %d, %d", A [ i ], A [ l ], A [ r ] );
        return 1;
      }
      else if ( A [ i ] + A [ l ] + A [ r ] < sum ) l ++;
      else r --;
    }
  }
  return 0;
}


