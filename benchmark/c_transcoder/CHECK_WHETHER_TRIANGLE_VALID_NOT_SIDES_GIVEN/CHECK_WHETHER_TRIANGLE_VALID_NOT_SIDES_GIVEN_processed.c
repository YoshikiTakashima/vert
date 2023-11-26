

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b, int c ) {
  if ( a + b <= c || a + c <= b || b + c <= a ) return 0;
  else return 1;
}


