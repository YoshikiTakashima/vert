

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int p ) {
  long long checkNumber = pow ( 2, p ) - 1;
  long long nextval = 4 % checkNumber;
  for ( int i = 1;
  i < p - 1;
  i ++ ) nextval = ( nextval * nextval - 2 ) % checkNumber;
  if(nextval == 0) return 1;
  else return 0;
}


