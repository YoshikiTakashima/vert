

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [], int K ) {
  int n = strlen(s);
  int C, c1 = 0, c2 = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( s [ i ] == 'a' ) c1 ++;
    if ( s [ i ] == 'b' ) {
      c2 ++;
      C += c1;
    }
  }
  return C * K + ( K * ( K - 1 ) / 2 ) * c1 * c2;
}


