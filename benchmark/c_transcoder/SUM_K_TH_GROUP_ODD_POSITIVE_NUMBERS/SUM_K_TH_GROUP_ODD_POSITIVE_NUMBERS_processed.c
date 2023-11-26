

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int k ) {
  int cur = ( k * ( k - 1 ) ) + 1;
  int sum = 0;
  while ( k -- ) {
    sum += cur;
    cur += 2;
  }
  return sum;
}


