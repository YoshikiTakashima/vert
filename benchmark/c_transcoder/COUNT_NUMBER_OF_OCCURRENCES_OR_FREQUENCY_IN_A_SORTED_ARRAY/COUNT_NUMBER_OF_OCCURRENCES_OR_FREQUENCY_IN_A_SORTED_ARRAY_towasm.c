

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

int f_gold ( int arr [ ], int n, int x ) {
  int res = 0;
  for ( int i = 0;
  i < n;
  i ++ ) if ( x == arr [ i ] ) res ++;
  return res;
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}