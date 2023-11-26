

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  char last = str [ 0 ];
  int i = 1, counter = 0;
  while ( i < len(str) ) {
    if ( str [ i ] == '0' && last == '1' ) {
      while ( str [ i ] == '0' ) i ++;
      if ( str [ i ] == '1' ) counter ++;
    }
    last = str [ i ];
    i ++;
  }
  return counter;
}


