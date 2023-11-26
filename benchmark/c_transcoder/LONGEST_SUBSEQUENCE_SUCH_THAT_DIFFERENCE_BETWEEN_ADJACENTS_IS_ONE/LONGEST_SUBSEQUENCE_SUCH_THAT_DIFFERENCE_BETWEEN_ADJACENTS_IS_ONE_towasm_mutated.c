

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
  int dp [ n ];
  for ( int i = 0;
  i < n;
  i ++ ) dp [ i ] = 1;
  for ( int i = 1;
  i < n;
  i ++ ) {
    for ( int j = 0;
    j < i;
    j ++ ) {
      if ( ( arr [ i ] == arr [ j ] + 1 ) || ( arr [ i ] == arr [ j ] - 1 ) ) dp [ i ] = max ( dp [ i ], dp [ j ] + 1 );
    }
  }
  int result = 1;
  for ( int i = 0;
  i < n;
  i ++ ) if ( result < dp [ i ] ) result = dp [ i ];
  return result;
}


int main(void) {
		int xv[] = {11,129};
	f_gold(xv,29);
}