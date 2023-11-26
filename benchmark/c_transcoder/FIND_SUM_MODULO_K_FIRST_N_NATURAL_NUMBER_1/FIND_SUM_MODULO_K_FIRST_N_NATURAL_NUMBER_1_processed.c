

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int N, int K ) {
  int ans = 0;
  int y = N / K;
  int x = N % K;
  ans = ( K * ( K - 1 ) / 2 ) * y + ( x * ( x + 1 ) ) / 2;
  return ans;
}


