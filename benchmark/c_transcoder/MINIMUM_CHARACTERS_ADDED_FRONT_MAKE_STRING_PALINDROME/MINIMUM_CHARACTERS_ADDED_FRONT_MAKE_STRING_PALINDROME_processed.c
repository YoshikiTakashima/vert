

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [] ) {
  int l = strlen(s);
  int j;
  for ( int i = 0, j = l - 1;
  i <= j;
  i ++, j -- ) {
    if ( s [ i ] != s [ j ] ) return 0;
  }
  return 1;
}


