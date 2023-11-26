

#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>

int min(int x, int y) { return (x < y)? x: y; }
int max(int x, int y) { return (x > y)? x: y; }
int cmpfunc (const void * a, const void * b) {return ( *(int*)a - *(int*)b );}
int len (int arr [ ]) {return ((int) (sizeof (arr) / sizeof (arr)[0]));}
void sort (int arr [ ], int n) {qsort (arr, n, sizeof(int), cmpfunc);}

int f_gold ( int n ) {
  int DP [ n + 1 ];
  DP [ 0 ] = 0;
  DP [ 1 ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    if ( i % 2 == 0 ) DP [ i ] = DP [ i / 2 ];
    else DP [ i ] = DP [ ( i - 1 ) / 2 ] + DP [ ( i + 1 ) / 2 ];
  }
  return DP [ n ];
}


int main(void) {
		f_gold(1);
}