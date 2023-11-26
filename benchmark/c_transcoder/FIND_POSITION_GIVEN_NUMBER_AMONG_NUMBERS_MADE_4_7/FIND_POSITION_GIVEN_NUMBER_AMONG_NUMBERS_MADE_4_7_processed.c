

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char n [] ) {
  int i = 0, pos = 0;
  while ( n [ i ] != '\0' ) {
    switch ( n [ i ] ) {
      case '4' : pos = pos * 2 + 1;
      break;
      case '7' : pos = pos * 2 + 2;
      break;
    }
    i ++;
  }
  return pos;
}


