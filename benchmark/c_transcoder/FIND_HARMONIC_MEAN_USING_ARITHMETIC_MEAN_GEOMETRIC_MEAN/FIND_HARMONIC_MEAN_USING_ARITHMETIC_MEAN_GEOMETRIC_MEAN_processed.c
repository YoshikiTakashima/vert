

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



float f_gold ( int a, int b ) {
  double AM, GM, HM;
  AM = ( a + b ) / 2;
  GM = sqrt ( a * b );
  HM = ( GM * GM ) / AM;
  return HM;
}


