

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

long f_gold ( int n ) {
  long dp [ 2 ] [ n + 1 ];
  dp [ 0 ] [ 1 ] = 1;
  dp [ 1 ] [ 1 ] = 2;
  for ( int i = 2;
  i <= n;
  i ++ ) {
    dp [ 0 ] [ i ] = dp [ 0 ] [ i - 1 ] + dp [ 1 ] [ i - 1 ];
    dp [ 1 ] [ i ] = dp [ 0 ] [ i - 1 ] * 2 + dp [ 1 ] [ i - 1 ];
  }
  return dp [ 0 ] [ n ] + dp [ 1 ] [ n ];
}


int main(void) {
		f_gold(29);
}