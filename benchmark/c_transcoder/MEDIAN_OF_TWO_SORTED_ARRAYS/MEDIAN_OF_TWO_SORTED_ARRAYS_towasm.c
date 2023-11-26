

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

int f_gold ( int ar1 [ ], int ar2 [ ], int n ) {
  int i = 0;
  int j = 0;
  int count;
  int m1 = - 1, m2 = - 1;
  for ( count = 0;
  count <= n;
  count ++ ) {
    if ( i == n ) {
      m1 = m2;
      m2 = ar2 [ 0 ];
      break;
    }
    else if ( j == n ) {
      m1 = m2;
      m2 = ar1 [ 0 ];
      break;
    }
    if ( ar1 [ i ] < ar2 [ j ] ) {
      m1 = m2;
      m2 = ar1 [ i ];
      i ++;
    }
    else {
      m1 = m2;
      m2 = ar2 [ j ];
      j ++;
    }
  }
  return ( m1 + m2 ) / 2;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3);
}