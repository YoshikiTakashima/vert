

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

int f_gold ( int arr1 [ ], int n, int arr2 [ ], int m ) {
  int table [ m ];
  for ( int j = 0;
  j < m;
  j ++ ) table [ j ] = 0;
  for ( int i = 0;
  i < n;
  i ++ ) {
    int current = 0;
    for ( int j = 0;
    j < m;
    j ++ ) {
      if ( arr1 [ i ] == arr2 [ j ] ) if ( current + 1 > table [ j ] ) table [ j ] = current + 1;
      if ( arr1 [ i ] > arr2 [ j ] ) if ( table [ j ] > current ) current = table [ j ];
    }
  }
  int result = 0;
  for ( int i = 0;
  i < m;
  i ++ ) if ( table [ i ] > result ) result = table [ i ];
  return result;
}


int main(void) {
		int xv[] = {11,12};
	int qe[] = {11,12};
	f_gold(xv,2,qe,29);
}