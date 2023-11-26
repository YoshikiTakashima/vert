

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( char s1 [], char s2 [] ) {
  int n = strlen(s1);
  int m = strlen(s2);
  bool dp [ n + 1 ] [ m + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) {
    for ( int j = 0;
    j <= m;
    j ++ ) {
      dp [ i ] [ j ] = 0;
    }
  }
  dp [ 0 ] [ 0 ] = 1;
  for ( int i = 0;
  i < strlen(s1);
  i ++ ) {
    for ( int j = 0;
    j <= strlen(s2);
    j ++ ) {
      if ( dp [ i ] [ j ] ) {
        if ( j < strlen(s2) && ( toupper ( s1 [ i ] ) == s2 [ j ] ) ) dp [ i + 1 ] [ j + 1 ] = 1;
        if ( ! isupper ( s1 [ i ] ) ) dp [ i + 1 ] [ j ] = 1;
      }
    }
  }
  return ( dp [ n ] [ m ] );
}


