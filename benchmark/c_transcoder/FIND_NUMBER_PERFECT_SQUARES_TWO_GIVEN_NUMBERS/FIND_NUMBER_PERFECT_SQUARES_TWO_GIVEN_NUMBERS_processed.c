

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a, int b ) {
  int cnt = 0;
  for ( int i = a;
  i <= b;
  i ++ ) for ( int j = 1;
  j * j <= i;
  j ++ ) if ( j * j == i ) cnt ++;
  return cnt;
}


