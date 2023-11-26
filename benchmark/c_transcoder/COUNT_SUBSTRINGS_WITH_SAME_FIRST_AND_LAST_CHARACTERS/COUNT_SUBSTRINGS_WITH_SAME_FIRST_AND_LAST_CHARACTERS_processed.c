

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [] ) {
  int result = 0;
  int n = strlen(s);
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = i;
  j < n;
  j ++ ) if ( s [ i ] == s [ j ] ) result ++;
  return result;
}


