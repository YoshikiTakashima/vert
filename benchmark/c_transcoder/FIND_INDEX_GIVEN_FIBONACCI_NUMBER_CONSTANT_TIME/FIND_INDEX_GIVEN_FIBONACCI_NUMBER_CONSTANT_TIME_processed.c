

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n <= 1 ) return n;
  int a = 0, b = 1, c = 1;
  int res = 1;
  while ( c < n ) {
    c = a + b;
    res ++;
    a = b;
    b = c;
  }
  return res;
}


