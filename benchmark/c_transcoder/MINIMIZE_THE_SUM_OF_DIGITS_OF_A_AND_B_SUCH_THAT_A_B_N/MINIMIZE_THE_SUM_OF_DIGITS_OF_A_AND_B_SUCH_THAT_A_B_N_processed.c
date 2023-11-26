

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int sum = 0;
  while ( n > 0 ) {
    sum += ( n % 10 );
    n /= 10;
  }
  if ( sum == 1 ) return 10;
  return sum;
}


