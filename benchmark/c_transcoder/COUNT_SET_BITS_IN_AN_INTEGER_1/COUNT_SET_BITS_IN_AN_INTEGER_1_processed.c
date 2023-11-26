

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n == 0 ) return 0;
  else return ( n & 1 ) + f_gold ( n >> 1 );
}


