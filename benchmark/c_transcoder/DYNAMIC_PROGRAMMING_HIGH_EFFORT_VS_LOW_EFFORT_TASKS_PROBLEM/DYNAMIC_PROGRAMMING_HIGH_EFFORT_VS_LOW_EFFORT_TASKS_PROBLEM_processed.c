

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int high [ ], int low [ ], int n ) {
  if ( n <= 0 ) return 0;
  return max ( high [ n - 1 ] + f_gold ( high, low, ( n - 2 ) ), low [ n - 1 ] + f_gold ( high, low, ( n - 1 ) ) );
}


