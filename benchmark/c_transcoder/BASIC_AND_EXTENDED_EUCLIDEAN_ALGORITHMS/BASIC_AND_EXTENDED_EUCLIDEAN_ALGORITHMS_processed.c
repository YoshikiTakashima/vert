

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b ) {
  if ( a == 0 ) return b;
  return f_gold ( b % a, a );
}


