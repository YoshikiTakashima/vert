

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, int y ) {
  while ( y != 0 ) {
    int carry = x & y;
    x = x ^ y;
    y = carry << 1;
  }
  return x;
}


