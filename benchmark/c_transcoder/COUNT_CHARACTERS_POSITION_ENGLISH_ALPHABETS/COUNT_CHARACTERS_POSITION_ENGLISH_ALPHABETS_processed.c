

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int result = 0;
  for ( int i = 0;
  i < len(str);
  i ++ ) if ( i == ( str [ i ] - 'a' ) || i == ( str [ i ] - 'A' ) ) result ++;
  return result;
}


