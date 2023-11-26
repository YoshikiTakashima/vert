

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



unsigned int f_gold ( unsigned int n ) {
  unsigned int count = 0;
  while ( n ) {
    count += n & 1;
    n >>= 1;
  }
  return count;
}


