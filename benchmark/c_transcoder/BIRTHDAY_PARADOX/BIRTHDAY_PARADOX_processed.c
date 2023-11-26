

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( double p ) {
  return ceil ( sqrt ( 2 * 365 * log ( 1 / ( 1 - p ) ) ) );
}


