

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [] ) {
  int n = strlen(s);
  for ( int i = 1;
  i < n;
  i ++ ) if ( s [ i ] != s [ 0 ] ) return 0;
  return 1;
}


