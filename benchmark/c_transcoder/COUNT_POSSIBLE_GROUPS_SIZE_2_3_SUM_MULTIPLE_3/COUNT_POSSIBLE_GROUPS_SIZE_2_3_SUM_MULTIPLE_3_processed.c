

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>



int f_gold ( int arr [ ], int n ) {
  int c [ 3 ] = {
    0 },
    i;
    int res = 0;
    for ( i = 0;
    i < n;
    i ++ ) c [ arr [ i ] % 3 ] ++;
    res += ( ( c [ 0 ] * ( c [ 0 ] - 1 ) ) >> 1 );
    res += c [ 1 ] * c [ 2 ];
    res += ( c [ 0 ] * ( c [ 0 ] - 1 ) * ( c [ 0 ] - 2 ) ) / 6;
    res += ( c [ 1 ] * ( c [ 1 ] - 1 ) * ( c [ 1 ] - 2 ) ) / 6;
    res += ( ( c [ 2 ] * ( c [ 2 ] - 1 ) * ( c [ 2 ] - 2 ) ) / 6 );
    res += c [ 0 ] * c [ 1 ] * c [ 2 ];
    return res;
  }
  

