

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int x, int y ) {
  int res = 1;
  for ( int i = 0;
  i < y;
  i ++ ) res = ( res * x ) % 10;
  return res;
}


