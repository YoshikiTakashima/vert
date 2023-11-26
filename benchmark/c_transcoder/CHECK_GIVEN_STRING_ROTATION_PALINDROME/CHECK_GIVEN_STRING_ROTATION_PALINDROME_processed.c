

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int l = 0;
  int h = strlen(str) - 1;
  while ( h > l ) if ( str [ l ++ ] != str [ h -- ] ) return 0;
  return 1;
}


