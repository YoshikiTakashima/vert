

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int n ) {
  if ( n == 0 || n == 1 ) return 1;
  return n * f_gold ( n - 2 );
}


