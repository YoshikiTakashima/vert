

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int process, int need ) {
  int minResources = 0;
  minResources = process * ( need - 1 ) + 1;
  return minResources;
}


