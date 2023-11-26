

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x1, int y1, int x2, int y2 ) {
  return ( x1 * ( y2 - y1 ) == y1 * ( x2 - x1 ) );
}


