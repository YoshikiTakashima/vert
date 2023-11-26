

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int n ) {
  int count = 0;
  for ( int curr = 19;
  ;
  curr += 9 ) {
    int sum = 0;
    for ( int x = curr;
    x > 0;
    x = x / 10 ) sum = sum + x % 10;
    if ( sum == 10 ) count ++;
    if ( count == n ) return curr;
  }
  return - 1;
}


