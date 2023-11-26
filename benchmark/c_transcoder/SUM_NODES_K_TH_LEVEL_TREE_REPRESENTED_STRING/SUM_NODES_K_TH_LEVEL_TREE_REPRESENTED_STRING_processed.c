

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char tree [], int k ) {
  int level = - 1;
  int sum = 0;
  int n = strlen(tree);
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( tree [ i ] == '(' ) level ++;
    else if ( tree [ i ] == ')' ) level --;
    else {
      if ( level == k ) sum += ( tree [ i ] - '0' );
    }
  }
  return sum;
}


