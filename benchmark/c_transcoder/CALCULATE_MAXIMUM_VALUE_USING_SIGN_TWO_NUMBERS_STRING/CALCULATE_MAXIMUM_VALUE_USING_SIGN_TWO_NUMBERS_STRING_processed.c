

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int res = str [ 0 ] - '0';
  for ( int i = 1;
  i < strlen(str);
  i ++ ) {
    if ( str [ i ] == '0' || str [ i ] == '1' || res < 2 ) res += ( str [ i ] - '0' );
    else res *= ( str [ i ] - '0' );
  }
  return res;
}


