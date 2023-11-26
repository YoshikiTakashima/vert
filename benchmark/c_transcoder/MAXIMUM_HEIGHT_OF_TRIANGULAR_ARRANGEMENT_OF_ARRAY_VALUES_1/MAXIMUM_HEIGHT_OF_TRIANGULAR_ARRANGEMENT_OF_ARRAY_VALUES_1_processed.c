

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  return floor ( ( - 1 + sqrt ( 1 + ( 8 * n ) ) ) / 2 );
}


