

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
  sort ( arr, arr + n );
  int minXor = INT_MAX;
  int val = 0;
  for ( int i = 0;
  i < n - 1;
  i ++ ) {
    val = arr [ i ] ^ arr [ i + 1 ];
    minXor = min ( minXor, val );
  }
  return minXor;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}