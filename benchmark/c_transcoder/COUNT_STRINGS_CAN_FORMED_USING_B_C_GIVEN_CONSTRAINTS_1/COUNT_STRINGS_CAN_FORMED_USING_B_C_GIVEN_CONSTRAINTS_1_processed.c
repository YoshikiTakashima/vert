

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  return 1 + ( n * 2 ) + ( n * ( ( n * n ) - 1 ) / 2 );
}


