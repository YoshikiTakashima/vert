

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char tree [], int k ) {
  int level = - 1;
  int product = 1;
  int n = strlen(tree);
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( tree [ i ] == '(' ) level ++;
    else if ( tree [ i ] == ')' ) level --;
    else {
      if ( level == k ) product *= ( tree [ i ] - '0' );
    }
  }
  return product;
}


