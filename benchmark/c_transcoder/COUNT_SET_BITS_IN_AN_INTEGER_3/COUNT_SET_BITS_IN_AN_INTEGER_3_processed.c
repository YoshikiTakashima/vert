

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n == 0 ) return 0;
  else return 1 + f_gold ( n & ( n - 1 ) );
}


