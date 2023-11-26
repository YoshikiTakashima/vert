

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int N ) {
  return ceil ( log2 ( N + 1 ) ) - 1;
}


