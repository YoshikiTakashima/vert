

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [], char c ) {
  int res = 0;
  for ( int i = 0;
  i < strlen(s);
  i ++ ) if ( s [ i ] == c ) res ++;
  return res;
}


