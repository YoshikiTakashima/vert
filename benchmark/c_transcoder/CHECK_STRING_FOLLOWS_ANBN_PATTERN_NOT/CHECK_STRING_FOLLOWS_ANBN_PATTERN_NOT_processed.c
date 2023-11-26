

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int n = strlen(str);
  int i;
  for ( i = 0;
  i < n;
  i ++ ) if ( str [ i ] != 'a' ) break;
  if ( i * 2 != n ) return 0;
  int j;
  for ( j = i;
  j < n;
  j ++ ) if ( str [ j ] != 'b' ) return 0;
  return 1;
}


