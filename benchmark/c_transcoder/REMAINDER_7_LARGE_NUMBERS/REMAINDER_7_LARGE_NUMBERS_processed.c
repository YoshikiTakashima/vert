

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char num [] ) {
  int series [ ] = {
    1, 3, 2, - 1, - 3, - 2 };
    int series_index = 0;
    int result = 0;
    for ( int i = len(num) - 1;
    i >= 0;
    i -- ) {
      int digit = num [ i ] - '0';
      result += digit * series [ series_index ];
      series_index = ( series_index + 1 ) % 6;
      result %= 7;
    }
    if ( result < 0 ) result = ( result + 7 ) % 7;
    return result;
  }
  

