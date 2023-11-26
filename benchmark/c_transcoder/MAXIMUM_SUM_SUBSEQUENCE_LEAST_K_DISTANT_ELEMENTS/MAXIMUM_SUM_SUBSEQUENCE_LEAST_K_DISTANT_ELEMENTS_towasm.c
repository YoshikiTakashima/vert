

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

int f_gold ( int arr [ ], int N, int k ) {
  int MS [ N ];
  MS [ N - 1 ] = arr [ N - 1 ];
  for ( int i = N - 2;
  i >= 0;
  i -- ) {
    if ( i + k + 1 >= N ) MS [ i ] = max ( arr [ i ], MS [ i + 1 ] );
    else MS [ i ] = max ( arr [ i ] + MS [ i + k + 1 ], MS [ i + 1 ] );
  }
  return MS [ 0 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}