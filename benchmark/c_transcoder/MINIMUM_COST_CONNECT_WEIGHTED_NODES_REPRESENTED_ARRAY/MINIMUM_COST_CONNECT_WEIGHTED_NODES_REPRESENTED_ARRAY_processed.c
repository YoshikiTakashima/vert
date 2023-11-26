

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  int mn = INT_MAX;
  int sum = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    mn = min ( a [ i ], mn );
    sum += a [ i ];
  }
  return mn * ( sum - mn );
}


