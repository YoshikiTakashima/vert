

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b ) {
  return ( a / 2 ) + ( b / 2 ) + ( ( a % 2 + b % 2 ) / 2 );
}


