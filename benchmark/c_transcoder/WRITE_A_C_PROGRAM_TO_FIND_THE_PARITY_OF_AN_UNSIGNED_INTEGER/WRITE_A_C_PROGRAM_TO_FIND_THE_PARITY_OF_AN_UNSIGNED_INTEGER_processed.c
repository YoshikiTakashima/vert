

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( unsigned int n ) {
  bool parity = 0;
  while ( n ) {
    parity = ! parity;
    n = n & ( n - 1 );
  }
  return parity;
}


