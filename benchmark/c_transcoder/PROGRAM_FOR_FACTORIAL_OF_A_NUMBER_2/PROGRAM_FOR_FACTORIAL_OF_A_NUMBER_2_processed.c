

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  return ( n == 1 || n == 0 ) ? 1 : n * f_gold ( n - 1 );
}


