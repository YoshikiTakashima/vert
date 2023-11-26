

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int len = strlen(str);
  int num, rem = 0;
  for ( int i = 0;
  i < len;
  i ++ ) {
    num = rem * 10 + ( str [ i ] - '0' );
    rem = num % 11;
  }
  return rem;
}


