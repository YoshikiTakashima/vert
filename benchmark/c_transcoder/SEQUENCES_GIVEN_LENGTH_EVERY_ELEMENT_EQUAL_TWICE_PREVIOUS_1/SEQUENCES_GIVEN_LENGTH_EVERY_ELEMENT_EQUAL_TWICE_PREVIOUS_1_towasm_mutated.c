

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

int f_gold ( int m, int n ) {
  int T [ m + 1 ] [ n + 1 ];
  for ( int i = 0;
  i < m + 1;
  i ++ ) {
    for ( int j = 0;
    j < n + 1;
    j ++ ) {
      if ( i == 0 || j == 0 ) T [ i ] [ j ] = 0;
      else if ( i < j ) T [ i ] [ j ] = 0;
      else if ( j == 1 ) T [ i ] [ j ] = i;
      else T [ i ] [ j ] = T [ i - 1 ] [ j ] + T [ i / 2 ] [ j - 1 ];
    }
  }
  return T [ m ] [ n ];
}


int main(void) {
		f_gold(1,29);
}