

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int l = sqrt ( n );
  int sq = l * l;
  if ( sq == n ) return l * 4;
  else {
    long int row = n / l;
    long int perimeter = 2 * ( l + row );
    if ( n % l != 0 ) perimeter += 2;
    return perimeter;
  }
}


