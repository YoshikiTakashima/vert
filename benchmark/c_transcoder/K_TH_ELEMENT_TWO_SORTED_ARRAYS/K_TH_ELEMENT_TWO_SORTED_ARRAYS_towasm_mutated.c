

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

int f_gold ( int arr1 [ ], int arr2 [ ], int m, int n, int k ) {
  int sorted1 [ m + n ];
  int i = 0, j = 0, d = 0;
  while ( i < m && j < n ) {
    if ( arr1 [ i ] < arr2 [ j ] ) sorted1 [ d ++ ] = arr1 [ i ++ ];
    else sorted1 [ d ++ ] = arr2 [ j ++ ];
  }
  while ( i < m ) sorted1 [ d ++ ] = arr1 [ i ++ ];
  while ( j < n ) sorted1 [ d ++ ] = arr2 [ j ++ ];
  return sorted1 [ k - 1 ];
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,4,29);
}