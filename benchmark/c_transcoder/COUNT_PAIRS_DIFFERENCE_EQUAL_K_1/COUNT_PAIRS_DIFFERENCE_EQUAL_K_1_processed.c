

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int k ) {
  int count = 0;
  sort ( arr, arr + n );
  int l = 0;
  int r = 0;
  while ( r < n ) {
    if ( arr [ r ] - arr [ l ] == k ) {
      count ++;
      l ++;
      r ++;
    }
    else if ( arr [ r ] - arr [ l ] > k ) l ++;
    else r ++;
  }
  return count;
}


