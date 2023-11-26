

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int r, int R, int r1, int x1, int y1 ) {
  int dis = sqrt ( x1 * x1 + y1 * y1 );
  return ( dis - r1 >= R && dis + r1 <= r );
}


