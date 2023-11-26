

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b, int k ) {
  int p = pow ( a, b );
  int count = 0;
  while ( p > 0 && count < k ) {
    int rem = p % 10;
    count ++;
    if ( count == k ) return rem;
    p = p / 10;
  }
  return 0;
}


