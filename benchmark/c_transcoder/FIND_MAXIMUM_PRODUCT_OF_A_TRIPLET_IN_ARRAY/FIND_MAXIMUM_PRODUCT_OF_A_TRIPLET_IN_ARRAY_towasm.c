

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

int f_gold ( int arr [ ], int n ) {
  if ( n < 3 ) return - 1;
  int max_product = INT_MIN;
  for ( int i = 0;
  i < n - 2;
  i ++ ) for ( int j = i + 1;
  j < n - 1;
  j ++ ) for ( int k = j + 1;
  k < n;
  k ++ ) max_product = max ( max_product, arr [ i ] * arr [ j ] * arr [ k ] );
  return max_product;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2);
}