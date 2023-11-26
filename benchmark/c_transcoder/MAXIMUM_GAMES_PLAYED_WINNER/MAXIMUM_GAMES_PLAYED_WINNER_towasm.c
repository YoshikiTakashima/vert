

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

int f_gold ( int N ) {
  int dp [ N ];
  dp [ 0 ] = 1;
  dp [ 1 ] = 2;
  int i = 2;
  do {
    dp [ i ] = dp [ i - 1 ] + dp [ i - 2 ];
  }
  while ( dp [ i ++ ] <= N );
  return ( i - 2 );
}


int main(void) {
		f_gold(1);
}