

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( unsigned int n ) {
  return n & ( n - 1 );
}


