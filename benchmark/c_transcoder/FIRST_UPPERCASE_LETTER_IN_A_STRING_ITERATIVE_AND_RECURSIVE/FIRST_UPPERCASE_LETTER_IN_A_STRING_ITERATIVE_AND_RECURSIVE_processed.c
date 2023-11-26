

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



char f_gold ( char str [] ) {
  for ( int i = 0;
  i < strlen(str);
  i ++ ) if ( isupper ( str [ i ] ) ) return str [ i ];
  return 0;
}


