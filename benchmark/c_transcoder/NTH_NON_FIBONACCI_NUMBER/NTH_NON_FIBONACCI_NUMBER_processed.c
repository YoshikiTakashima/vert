

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int prevPrev = 1, prev = 2, curr = 3;
  while ( n > 0 ) {
    prevPrev = prev;
    prev = curr;
    curr = prevPrev + prev;
    n = n - ( curr - prev - 1 );
  }
  n = n + ( curr - prev - 1 );
  return prev + n;
}


