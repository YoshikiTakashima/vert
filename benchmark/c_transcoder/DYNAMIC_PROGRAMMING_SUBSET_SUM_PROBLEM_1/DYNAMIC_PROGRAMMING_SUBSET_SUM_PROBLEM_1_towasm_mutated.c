

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

int f_gold ( int set [ ], int n, int sum ) {
  bool subset [ n + 1 ] [ sum + 1 ];
  for ( int i = 0;
  i <= n;
  i ++ ) subset [ i ] [ 0 ] = 1;
  for ( int i = 1;
  i <= sum;
  i ++ ) subset [ 0 ] [ i ] = 0;
  for ( int i = 1;
  i <= n;
  i ++ ) {
    for ( int j = 1;
    j <= sum;
    j ++ ) {
      if ( j < set [ i - 1 ] ) subset [ i ] [ j ] = subset [ i - 1 ] [ j ];
      if ( j >= set [ i - 1 ] ) subset [ i ] [ j ] = subset [ i - 1 ] [ j ] || subset [ i - 1 ] [ j - set [ i - 1 ] ];
    }
  }
  return subset [ n ] [ sum ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,29);
}