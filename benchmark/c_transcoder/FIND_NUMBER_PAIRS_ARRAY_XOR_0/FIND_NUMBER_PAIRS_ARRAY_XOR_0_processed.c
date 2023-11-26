

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  sort ( a, a + n );
  int count = 1;
  int answer = 0;
  for ( int i = 1;
  i < n;
  i ++ ) {
    if ( a [ i ] == a [ i - 1 ] ) {
      count += 1;
    }
    else {
      answer = answer + ( count * ( count - 1 ) ) / 2;
      count = 1;
    }
  }
  answer = answer + ( count * ( count - 1 ) ) / 2;
  return answer;
}


