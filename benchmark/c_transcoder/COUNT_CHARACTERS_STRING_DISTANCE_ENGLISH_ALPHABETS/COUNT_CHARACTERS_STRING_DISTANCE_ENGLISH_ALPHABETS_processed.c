

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int result = 0;
  int n = strlen(str);
  for ( int i = 0;
  i < n;
  i ++ ) for ( int j = i + 1;
  j < n;
  j ++ ) if ( abs ( str [ i ] - str [ j ] ) == abs ( i - j ) ) result ++;
  return result;
}


