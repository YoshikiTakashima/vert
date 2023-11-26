

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int i, j;
  int leftsum, rightsum;
  for ( i = 0;
  i < n;
  ++ i ) {
    leftsum = 0;
    for ( j = 0;
    j < i;
    j ++ ) leftsum += arr [ j ];
    rightsum = 0;
    for ( j = i + 1;
    j < n;
    j ++ ) rightsum += arr [ j ];
    if ( leftsum == rightsum ) return i;
  }
  return - 1;
}


