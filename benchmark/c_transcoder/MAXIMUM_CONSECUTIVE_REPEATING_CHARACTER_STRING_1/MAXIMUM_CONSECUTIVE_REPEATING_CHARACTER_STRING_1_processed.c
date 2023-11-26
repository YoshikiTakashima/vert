

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



char f_gold ( char str [] ) {
  int n = strlen(str);
  int count = 0;
  char res = str [ 0 ];
  int cur_count = 1;
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( i < n - 1 && str [ i ] == str [ i + 1 ] ) cur_count ++;
    else {
      if ( cur_count > count ) {
        count = cur_count;
        res = str [ i ];
      }
      cur_count = 1;
    }
  }
  return res;
}


