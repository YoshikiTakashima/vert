

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int base ) {
  base = ( base - 2 );
  base = base / 2;
  return base * ( base + 1 ) / 2;
}


