

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b ) {
  if ( a == b ) return a;
  if ( a == 0 ) return b;
  if ( b == 0 ) return a;
  if ( ~ a & 1 ) {
    if ( b & 1 ) return f_gold ( a >> 1, b );
    else return f_gold ( a >> 1, b >> 1 ) << 1;
  }
  if ( ~ b & 1 ) return f_gold ( a, b >> 1 );
  if ( a > b ) return f_gold ( ( a - b ) >> 1, b );
  return f_gold ( ( b - a ) >> 1, a );
}


