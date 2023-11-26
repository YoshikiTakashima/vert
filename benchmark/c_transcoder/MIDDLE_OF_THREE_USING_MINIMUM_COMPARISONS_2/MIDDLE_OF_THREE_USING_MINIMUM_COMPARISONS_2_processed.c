

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b, int c ) {
  int x = a - b;
  int y = b - c;
  int z = a - c;
  if ( x * y > 0 ) return b;
  else if ( x * z > 0 ) return c;
  else return a;
}


