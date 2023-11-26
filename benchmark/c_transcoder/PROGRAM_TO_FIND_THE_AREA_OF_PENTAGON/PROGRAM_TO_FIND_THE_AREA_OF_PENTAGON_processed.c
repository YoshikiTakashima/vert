

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



float f_gold ( float a ) {
  float area;
  area = ( sqrt ( 5 * ( 5 + 2 * ( sqrt ( 5 ) ) ) ) * a * a ) / 4;
  return area;
}


