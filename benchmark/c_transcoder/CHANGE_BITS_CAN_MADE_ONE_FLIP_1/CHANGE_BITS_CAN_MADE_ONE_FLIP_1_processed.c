

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char str [] ) {
  int sum = 0;
  int n = strlen(str);
  for ( int i = 0;
  i < n;
  i ++ ) sum += str [ i ] - '0';
  return ( sum == n - 1 || sum == 1 );
}


