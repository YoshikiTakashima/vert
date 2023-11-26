

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str1 [], char str2 [], int k ) {
  if ( ( strlen(str1) + strlen(str2) ) < k ) return 1;
  int commonLength = 0;
  for ( int i = 0;
  i < min ( strlen(str1), strlen(str2) );
  i ++ ) {
    if ( str1 [ i ] == str2 [ i ] ) commonLength ++;
    else break;
  }
  if ( ( k - strlen(str1) - strlen(str2) + 2 * commonLength ) % 2 == 0 ) return 1;
  return 0;
}


