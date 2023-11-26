

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int ans = 0;
  for ( int length = 1;
  length <= sqrt ( n );
  ++ length ) for ( int height = length;
  height * length <= n;
  ++ height ) ans ++;
  return ans;
}


