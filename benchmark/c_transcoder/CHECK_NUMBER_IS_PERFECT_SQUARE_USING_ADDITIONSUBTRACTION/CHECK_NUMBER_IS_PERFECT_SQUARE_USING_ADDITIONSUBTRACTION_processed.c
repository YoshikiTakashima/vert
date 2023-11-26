

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  for ( int sum = 0, i = 1;
  sum < n;
  i += 2 ) {
    sum += i;
    if ( sum == n ) return 1;
  }
  return 0;
}


