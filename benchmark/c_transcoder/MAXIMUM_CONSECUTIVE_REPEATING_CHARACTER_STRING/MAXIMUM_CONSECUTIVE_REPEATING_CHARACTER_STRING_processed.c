

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



char f_gold ( char str [] ) {
  int len = strlen(str);
  int count = 0;
  char res = str [ 0 ];
  for ( int i = 0;
  i < len;
  i ++ ) {
    int cur_count = 1;
    for ( int j = i + 1;
    j < len;
    j ++ ) {
      if ( str [ i ] != str [ j ] ) break;
      cur_count ++;
    }
    if ( cur_count > count ) {
      count = cur_count;
      res = str [ i ];
    }
  }
  return res;
}


