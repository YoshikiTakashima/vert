

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int a [ ], int n ) {
  sort ( a, a + n );
  int num1 = 0;
  int num2 = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i % 2 == 0 ) num1 = num1 * 10 + a [ i ];
    else num2 = num2 * 10 + a [ i ];
  }
  return num2 + num1;
}


