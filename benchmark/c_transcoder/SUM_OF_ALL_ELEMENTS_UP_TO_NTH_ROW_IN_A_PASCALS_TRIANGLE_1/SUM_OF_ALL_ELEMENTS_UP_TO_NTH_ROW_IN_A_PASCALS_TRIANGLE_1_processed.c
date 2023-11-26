

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  long int sum = 0;
  sum = 1 << n;
  return ( sum - 1 );
}


