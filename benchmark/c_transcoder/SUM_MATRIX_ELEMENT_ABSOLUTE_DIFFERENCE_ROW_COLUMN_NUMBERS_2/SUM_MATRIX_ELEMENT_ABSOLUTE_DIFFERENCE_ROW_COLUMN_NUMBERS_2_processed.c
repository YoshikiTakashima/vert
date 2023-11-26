

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  n --;
  int sum = 0;
  sum += ( n * ( n + 1 ) ) / 2;
  sum += ( n * ( n + 1 ) * ( 2 * n + 1 ) ) / 6;
  return sum;
}


