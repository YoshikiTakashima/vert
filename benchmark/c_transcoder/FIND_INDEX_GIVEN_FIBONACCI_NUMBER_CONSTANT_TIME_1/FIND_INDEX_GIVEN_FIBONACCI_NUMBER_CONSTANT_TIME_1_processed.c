

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  float fibo = 2.078087 * log ( n ) + 1.672276;
  return round ( fibo );
}


