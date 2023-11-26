

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char X [], char Y [] ) {
  int m = strlen(X);
  int n = strlen(Y);
  int result = 0;
  int len [ 2 ] [ n ];
  int currRow = 0;
  for ( int i = 0;
  i <= m;
  i ++ ) {
    for ( int j = 0;
    j <= n;
    j ++ ) {
      if ( i == 0 || j == 0 ) {
        len [ currRow ] [ j ] = 0;
      }
      else if ( X [ i - 1 ] == Y [ j - 1 ] ) {
        len [ currRow ] [ j ] = len [ 1 - currRow ] [ j - 1 ] + 1;
        result = max ( result, len [ currRow ] [ j ] );
      }
      else {
        len [ currRow ] [ j ] = 0;
      }
    }
    currRow = 1 - currRow;
  }
  return result;
}


