

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

int f_gold ( int n, int m ) {
  int dp [ n + 1 ] [ m + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) dp [ i ] [ 0 ] = 1;
  for ( int i = 0;
  i <= m;
  i ++ ) dp [ 0 ] [ i ] = 1;
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = 1;
  j <= m;
  j ++ ) dp [ i ] [ j ] = dp [ i - 1 ] [ j ] + dp [ i ] [ j - 1 ];
  return dp [ n ] [ m ];
}


int main(void) {
		f_gold(1,2);
}