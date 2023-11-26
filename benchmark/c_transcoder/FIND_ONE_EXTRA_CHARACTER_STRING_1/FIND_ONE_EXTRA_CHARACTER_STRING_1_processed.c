

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



char f_gold ( char strA [], char strB [] ) {
  int res = 0, i;
  for ( i = 0;
  i < strlen(strA);
  i ++ ) {
    res ^= strA [ i ];
  }
  for ( i = 0;
  i < strlen(strB);
  i ++ ) {
    res ^= strB [ i ];
  }
  return ( ( char ) ( res ) );
}


