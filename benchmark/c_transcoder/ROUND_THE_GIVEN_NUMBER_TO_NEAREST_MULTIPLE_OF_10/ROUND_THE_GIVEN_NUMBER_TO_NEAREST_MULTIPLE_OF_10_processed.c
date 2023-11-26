

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int a = ( n / 10 ) * 10;
  int b = a + 10;
  return ( n - a > b - n ) ? b : a;
}


