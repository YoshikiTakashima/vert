

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int n ) {
  unsigned int p = 1;
  if ( n && ! ( n & ( n - 1 ) ) ) return n;
  while ( p < n ) p <<= 1;
  return p;
}


