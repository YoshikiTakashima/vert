

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n == 1 ) return 1;
  int z;
  float e = 2.71;
  z = sqrt ( 2 * 3.14 * n ) * pow ( ( n / e ), n );
  return z;
}


