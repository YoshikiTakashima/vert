

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned long long f_gold ( int n ) {
  return ( n * ( n + 1 ) / 2 ) * ( 1 << ( n - 1 ) );
}


