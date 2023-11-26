

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

int f_gold ( int A [ ], int B [ ], int m, int n ) {
  long int dp [ n + 1 ] [ m + 1 ];
  memset ( dp, 0, sizeof ( dp ) );
  for ( int i = 1;
  i <= n;
  i ++ ) for ( int j = i;
  j <= m;
  j ++ ) dp [ i ] [ j ] = max ( ( dp [ i - 1 ] [ j - 1 ] + ( A [ j - 1 ] * B [ i - 1 ] ) ), dp [ i ] [ j - 1 ] );
  return dp [ n ] [ m ];
}


int main(void) {
		int xv[] = {11,12};
	int yq[] = {11,12};
	f_gold(xv,yq,3,29);
}