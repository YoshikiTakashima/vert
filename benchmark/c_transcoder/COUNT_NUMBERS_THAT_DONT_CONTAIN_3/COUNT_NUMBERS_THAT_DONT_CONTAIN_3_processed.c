

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  if ( n < 3 ) return n;
  if ( n >= 3 && n < 10 ) return n - 1;
  int po = 1;
  while ( n / po > 9 ) po = po * 10;
  int msd = n / po;
  if ( msd != 3 ) return f_gold ( msd ) * f_gold ( po - 1 ) + f_gold ( msd ) + f_gold ( n % po );
  else return f_gold ( msd * po - 1 );
}


