

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

int f_gold ( int arr [ ], int n, int m ) {
  if ( n > m ) return 1;
  bool DP [ m ];
  memset ( DP, 0, m );
  for ( int i = 0;
  i < n;
  i ++ ) {
    if ( DP [ 0 ] ) return 1;
    bool temp [ m ];
    memset ( temp, 0, m );
    for ( int j = 0;
    j < m;
    j ++ ) {
      if ( DP [ j ] == 1 ) {
        if ( DP [ ( j + arr [ i ] ) % m ] == 0 ) temp [ ( j + arr [ i ] ) % m ] = 1;
      }
    }
    for ( int j = 0;
    j < m;
    j ++ ) if ( temp [ j ] ) DP [ j ] = 1;
    DP [ arr [ i ] % m ] = 1;
  }
  return DP [ 0 ];
}


int main(void) {
		int xv[] = {11,12};
	f_gold(xv,2,3);
}