

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int s ) {
  int maxvalue = 0;
  for ( int i = 1;
  i <= s - 2;
  i ++ ) {
    for ( int j = 1;
    j <= s - 1;
    j ++ ) {
      int k = s - i - j;
      maxvalue = max ( maxvalue, i * j * k );
    }
  }
  return maxvalue;
}


