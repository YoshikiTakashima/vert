

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

int f_gold ( int a1 [ ], int a2 [ ], int a3 [ ], int n1, int n2, int n3, int sum ) {
  for ( int i = 0;
  i < n1;
  i ++ ) for ( int j = 0;
  j < n2;
  j ++ ) for ( int k = 0;
  k < n3;
  k ++ ) if ( a1 [ i ] + a2 [ j ] + a3 [ k ] == sum ) return 1;
  return 0;
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	int qe[] = {11,12};
	f_gold(xv,yq,qe,4,5,6,7);
}