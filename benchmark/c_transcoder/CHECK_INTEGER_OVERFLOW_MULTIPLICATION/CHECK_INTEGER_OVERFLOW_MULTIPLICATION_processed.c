

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( long long a, long long b ) {
  if ( a == 0 || b == 0 ) return 0;
  long long result = a * b;
  if ( a == result / b ) return 0;
  else return 1;
}


