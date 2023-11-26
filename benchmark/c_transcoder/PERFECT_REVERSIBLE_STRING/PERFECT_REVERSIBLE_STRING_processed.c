

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int i = 0, j = strlen(str) - 1;
  while ( i < j ) {
    if ( str [ i ] != str [ j ] ) return 0;
    i ++;
    j --;
  }
  return 1;
}


