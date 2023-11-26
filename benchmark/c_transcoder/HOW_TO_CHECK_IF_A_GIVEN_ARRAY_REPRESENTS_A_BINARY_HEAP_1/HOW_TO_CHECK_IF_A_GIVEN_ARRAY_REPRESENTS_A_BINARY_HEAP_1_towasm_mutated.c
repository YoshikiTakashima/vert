

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
  for ( int i = 0;
  i <= ( n - 2 ) / 2;
  i ++ ) {
    if ( arr [ 2 * i + 1 ] > arr [ i ] ) return 0;
    if ( 2 * i + 2 < n && arr [ 2 * i + 2 ] > arr [ i ] ) return 0;
  }
  return 1;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}