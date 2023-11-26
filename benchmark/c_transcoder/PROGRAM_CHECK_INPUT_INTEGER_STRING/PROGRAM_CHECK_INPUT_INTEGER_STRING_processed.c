

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [] ) {
  for ( int i = 0;
  i < strlen(s);
  i ++ ) if ( isdigit ( s [ i ] ) == 0 ) return 0;
  return 1;
}


