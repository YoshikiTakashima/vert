

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( long n ) {
  int S = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) S += i * i - ( i - 1 ) * ( i - 1 );
  return S;
}


