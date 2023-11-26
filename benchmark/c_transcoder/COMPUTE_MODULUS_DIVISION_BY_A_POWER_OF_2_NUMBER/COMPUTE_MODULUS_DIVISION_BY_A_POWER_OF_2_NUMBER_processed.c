

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int n, unsigned int d ) {
  return ( n & ( d - 1 ) );
}


