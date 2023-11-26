

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



float f_gold ( int side ) {
  double volume = ( pow ( side, 3 ) / ( 6 * sqrt ( 2 ) ) );
  return volume;
}


