

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

int f_gold ( int arr [ ], int n, int key, int capacity ) {
  if ( n >= capacity ) return n;
  int i;
  for ( i = n - 1;
  ( i >= 0 && arr [ i ] > key );
  i -- ) arr [ i + 1 ] = arr [ i ];
  arr [ i + 1 ] = key;
  return ( n + 1 );
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3,4);
}