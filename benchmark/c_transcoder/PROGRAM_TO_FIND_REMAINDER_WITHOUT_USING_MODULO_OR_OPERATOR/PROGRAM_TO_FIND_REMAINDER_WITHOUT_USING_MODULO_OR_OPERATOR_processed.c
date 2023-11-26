

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int num, int divisor ) {
  return ( num - divisor * ( num / divisor ) );
}


