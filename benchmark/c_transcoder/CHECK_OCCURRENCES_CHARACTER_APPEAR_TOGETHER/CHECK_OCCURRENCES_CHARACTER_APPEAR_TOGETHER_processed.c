

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s [], char c ) {
  bool oneSeen = 0;
  int i = 0, n = strlen(s);
  while ( i < n ) {
    if ( s [ i ] == c ) {
      if ( oneSeen == 1 ) return 0;
      while ( i < n && s [ i ] == c ) i ++;
      oneSeen = 1;
    }
    else i ++;
  }
  return 1;
}


