

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  if ( n == 1 ) return 1;
  int i;
  for ( i = 1;
  i < n && arr [ i - 1 ] < arr [ i ];
  i ++ );
  if ( i == n ) return 1;
  int j = i;
  while ( arr [ j ] < arr [ j - 1 ] ) {
    if ( i > 1 && arr [ j ] < arr [ i - 2 ] ) return 0;
    j ++;
  }
  if ( j == n ) return 1;
  int k = j;
  if ( arr [ k ] < arr [ i - 1 ] ) return 0;
  while ( k > 1 && k < n ) {
    if ( arr [ k ] < arr [ k - 1 ] ) return 0;
    k ++;
  }
  return 1;
}


