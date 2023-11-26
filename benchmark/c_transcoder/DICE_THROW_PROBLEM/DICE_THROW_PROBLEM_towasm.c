

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

int f_gold ( int m, int n, int x ) {
  int table [ n + 1 ] [ x + 1 ];
  memset ( table, 0, sizeof ( table ) );
  for ( int j = 1;
  j <= m && j <= x;
  j ++ ) table [ 1 ] [ j ] = 1;
  for ( int i = 2;
  i <= n;
  i ++ ) for ( int j = 1;
  j <= x;
  j ++ ) for ( int k = 1;
  k <= m && k < j;
  k ++ ) table [ i ] [ j ] += table [ i - 1 ] [ j - k ];
  return table [ n ] [ x ];
}


int main(void) {
		f_gold(1,2,3);
}