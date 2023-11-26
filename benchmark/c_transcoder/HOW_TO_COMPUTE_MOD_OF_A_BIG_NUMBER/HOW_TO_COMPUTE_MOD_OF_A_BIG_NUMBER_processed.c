

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char num [], int a ) {
  int res = 0;
  for ( int i = 0;
  i < strlen(num);
  i ++ ) res = ( res * 10 + ( int ) num [ i ] - '0' ) % a;
  return res;
}


