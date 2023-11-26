

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n, int x, int k ) {
  int i = 0;
  while ( i < n ) {
    if ( arr [ i ] == x ) return i;
    i = i + max ( 1, abs ( arr [ i ] - x ) / k );
  }
  printf("number is not present!");
  return - 1;
}


