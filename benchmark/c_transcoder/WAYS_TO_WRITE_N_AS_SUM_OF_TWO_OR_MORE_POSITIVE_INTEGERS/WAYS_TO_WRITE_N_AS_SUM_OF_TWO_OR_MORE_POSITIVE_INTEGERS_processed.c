

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int table [ n + 1 ];
  memset ( table, 0, sizeof ( table ) );
  table [ 0 ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) for ( int j = i;
  j <= n;
  j ++ ) table [ j ] += table [ j - i ];
  return table [ n ];
}


