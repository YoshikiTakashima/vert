

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int n ) {
  unsigned count = 0;
  if ( n && ! ( n & ( n - 1 ) ) ) return n;
  while ( n != 0 ) {
    n >>= 1;
    count += 1;
  }
  return 1 << count;
}


