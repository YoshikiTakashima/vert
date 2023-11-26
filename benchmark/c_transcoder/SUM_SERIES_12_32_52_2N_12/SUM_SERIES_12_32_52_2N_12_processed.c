

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int sum = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) sum = sum + ( 2 * i - 1 ) * ( 2 * i - 1 );
  return sum;
}


