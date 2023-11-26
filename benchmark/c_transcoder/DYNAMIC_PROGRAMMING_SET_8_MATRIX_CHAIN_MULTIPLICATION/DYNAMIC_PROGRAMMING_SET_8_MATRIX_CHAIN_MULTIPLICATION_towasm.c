

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

int f_gold ( int p [ ], int i, int j ) {
  if ( i == j ) return 0;
  int k;
  int min = INT_MAX;
  int count;
  for ( k = i;
  k < j;
  k ++ ) {
    count = f_gold ( p, i, k ) + f_gold ( p, k + 1, j ) + p [ i - 1 ] * p [ k ] * p [ j ];
    if ( count < min ) min = count;
  }
  return min;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}