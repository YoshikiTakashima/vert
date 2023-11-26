

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b, int c, int d ) {
  int sum = a * a + b * b + c * c;
  if ( d * d == sum ) return 1;
  else return 0;
}


